mod cli;

use std::io::{self, BufRead, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anstream::{eprintln, print, println};
use anstyle::{AnsiColor, Style};
use anyhow::{Context, Result, bail};
use hatchzip_i18n::{self as i18n, Key};
use indicatif::{ProgressBar, ProgressStyle};

use cli::{
    Command, EXIT_DAMAGED, EXIT_FAILURE, EXIT_NO_SPACE, EXIT_PASSWORD, EXIT_PAUSED, Secret, Space,
    Target,
};
use hatchzip_core::archive::storage::Reads;
use hatchzip_core::archive::{Damaged, Kind, PasswordNeeded, WrongPassword};
use hatchzip_core::engine::{self, Job, Outcome, Progress, Run};
use hatchzip_core::human;
use hatchzip_core::journal::{Journal, Paths};
use hatchzip_core::session::{self, State, display_name, locate};

const ERROR: Style = AnsiColor::Red.on_default().bold();
const GOOD: Style = AnsiColor::Green.on_default().bold();
const BOLD: Style = Style::new().bold();
const DIM: Style = Style::new().dimmed();

static STOP: AtomicBool = AtomicBool::new(false);
static QUIET: AtomicBool = AtomicBool::new(false);

/// Normal output, silenced by `--quiet`. Errors and prompts always show.
macro_rules! say {
    ($($t:tt)*) => {
        if !QUIET.load(Ordering::Relaxed) {
            println!($($t)*);
        }
    };
}

fn main() -> ExitCode {
    // Help is printed here so its headings read in the language in use.
    let argv: Vec<std::ffi::OsString> = std::env::args_os().collect();
    if cli::print_help_if_asked(&cli::localized(), &argv) {
        return ExitCode::SUCCESS;
    }
    let cli = cli::parse();
    QUIET.store(cli.quiet, Ordering::Relaxed);
    let code = match run(cli.command) {
        Ok(code) => code,
        Err(e) => fail(&e),
    };
    ExitCode::from(code)
}

/// Prints an error and returns the exit status it calls for.
fn fail(e: &anyhow::Error) -> u8 {
    eprintln!(
        "{ERROR}{}{ERROR:#} {e:#}",
        Key::CLI_ERROR.text(i18n::current())
    );
    if e.is::<PasswordNeeded>() || e.is::<WrongPassword>() {
        if !io::stdin().is_terminal() {
            eprintln!("{}", Key::CLI_PASSWORD_NO_TTY.text(i18n::current()));
        }
        EXIT_PASSWORD
    } else if e.is::<Damaged>() {
        EXIT_DAMAGED
    } else {
        EXIT_FAILURE
    }
}

/// Opens the archive, asking for a password when it is encrypted and none
/// was given, and again (up to three times) when one was wrong. Nothing is
/// touched until the password is proven.
fn open(
    archive: &Path,
    output: Option<PathBuf>,
    writable: bool,
    secret: Secret,
) -> Result<(State, Paths)> {
    let (primary, paths) = locate(archive)?;
    let reads = Arc::new(Reads::default());
    let mut reading = Some(Reading::show(&reads));
    let mut tries = 0;
    let opened = session::open_encrypted_noting(
        &primary,
        &paths,
        output,
        writable,
        secret.password,
        &reads,
        |wrong| {
            // The bar would draw over the prompt.
            if let Some(r) = reading.take() {
                r.finish();
            }
            if wrong {
                eprintln!(
                    "{ERROR}{}{ERROR:#}",
                    Key::CLI_INCORRECT_PASSWORD.text(i18n::current())
                );
            }
            tries += 1;
            if tries > 3 || !io::stdin().is_terminal() {
                return None;
            }
            rpassword::prompt_password(Key::CLI_PASSWORD_PROMPT.text(i18n::current())).ok()
        },
        |file, size| {
            // Small files decode in a moment; say nothing then.
            if size >= 64 << 20 && !QUIET.load(Ordering::Relaxed) {
                eprintln!(
                    "{}",
                    i18n::fill2(
                        Key::CLI_CHECKING_PASSWORD.text(i18n::current()),
                        file.display(),
                        human(size)
                    )
                );
            }
        },
    );
    if let Some(r) = reading {
        r.finish();
    }
    let (state, _) = opened?;
    Ok((state, paths))
}

/// A bar for a long read through the archive while it opens: a compressed
/// tar is decoded in full before anything else, which takes minutes for a
/// large one. It appears only once opening has taken a moment, and only on
/// a terminal without `--quiet`.
struct Reading {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Reading {
    const DELAY: Duration = Duration::from_millis(500);

    fn show(reads: &Arc<Reads>) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let thread = (io::stderr().is_terminal() && !QUIET.load(Ordering::Relaxed)).then(|| {
            let (stop, reads) = (stop.clone(), reads.clone());
            std::thread::spawn(move || {
                let start = std::time::Instant::now();
                let mut bar: Option<ProgressBar> = None;
                while !stop.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(50));
                    let total = reads.total.load(Ordering::Relaxed);
                    let done = reads.done.load(Ordering::Relaxed).min(total);
                    if bar.is_none() && start.elapsed() >= Self::DELAY && done < total {
                        // No speed or time left: the bar starts part way
                        // through, which throws the estimate off for its
                        // first seconds.
                        let b = ProgressBar::new(total).with_position(done);
                        b.set_style(
                            ProgressStyle::with_template(
                                "{prefix} [{bar:32}] {binary_bytes}/{binary_total_bytes}  {percent}%",
                            )
                            .expect("valid template")
                            .progress_chars("=> "),
                        );
                        b.set_prefix(Key::CLI_READING_BAR.text(i18n::current()));
                        b.enable_steady_tick(Duration::from_millis(200));
                        bar = Some(b);
                    }
                    if let Some(b) = &bar {
                        b.set_length(total);
                        b.set_position(done);
                    }
                }
                if let Some(b) = bar {
                    b.finish_and_clear();
                }
            })
        });
        Self { stop, thread }
    }

    fn finish(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn run(command: Command) -> Result<u8> {
    match command {
        Command::Extract {
            target,
            keep,
            verify_first,
            space,
            yes,
        } => extract_all(target, keep.keep_archive, verify_first, space, yes),
        Command::Check {
            target,
            keep,
            space,
        } => check_all(target, keep.keep_archive, space),
        Command::List { archives, secret } => {
            let archives = distinct(&archives)?;
            Ok(each(&archives, true, |a| list(a, secret.clone())))
        }
        Command::Test { archives, secret } => {
            let archives = distinct(&archives)?;
            let code = each(&archives, true, |a| test_command(a, secret.clone()));
            if archives.len() > 1 && code == 0 {
                say!(
                    "{GOOD}{}{GOOD:#}",
                    i18n::fill1(Key::CLI_ALL_TESTED.text(i18n::current()), archives.len())
                );
            }
            Ok(code)
        }
        Command::Restore { archive } => restore(&archive),
        Command::Completions { shell } => {
            // Named after however we were called, so `hz completions bash`
            // completes the `hz` alias.
            let name = std::env::args_os()
                .next()
                .and_then(|a| {
                    Path::new(&a)
                        .file_stem()
                        .map(|s| s.to_string_lossy().into_owned())
                })
                .filter(|n| n == "hz")
                .unwrap_or_else(|| "hatchzip".into());
            clap_complete::generate(shell, &mut cli::localized(), name, &mut io::stdout());
            Ok(0)
        }
        Command::Man => {
            clap_mangen::Man::new(cli::localized()).render(&mut io::stdout())?;
            Ok(0)
        }
    }
}

/// The archives as given, minus repeats. Volumes of one split set count
/// once, so `game.7z.*` from a shell glob means the one archive.
fn distinct(archives: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for a in archives {
        if seen.insert(locate(a)?.0) {
            out.push(a.clone());
        }
    }
    Ok(out)
}

/// Runs `f` on every archive, even after one fails, and returns the first
/// status that is not success. With several archives, each one's output is
/// headed by its name when `headed`.
fn each(archives: &[PathBuf], headed: bool, mut f: impl FnMut(&Path) -> Result<u8>) -> u8 {
    let mut first = 0;
    for (i, a) in archives.iter().enumerate() {
        if archives.len() > 1 {
            if i > 0 {
                say!();
            }
            if headed {
                say!("{BOLD}{}{BOLD:#}", a.display());
            }
        }
        let code = f(a).unwrap_or_else(|e| fail(&e));
        if first == 0 {
            first = code;
        }
    }
    first
}

/// Prints the space plan and returns whether it fits. The verdict prints
/// even under `--quiet`, since it is the whole answer.
fn report(
    job: &Job,
    out: &Path,
    journal: Option<&Journal>,
    keep_free: u64,
    keep: bool,
) -> Result<bool> {
    let lang = i18n::current();
    let plan = session::plan(job, out, journal.is_some(), keep_free, keep)?;
    let left = job.remaining_size();

    say!(
        "{BOLD}{}{BOLD:#}  {} {}, {}",
        display_name(job),
        human(job.storage.original_len()),
        job.format_name,
        i18n::fill2(
            Key::CLI_REPORT_SIZE.text(lang),
            human(journal.map_or(left, |j| j.total_size)),
            entries(job.item_count())
        )
    );
    if let Some(j) = journal {
        say!(
            "{}",
            i18n::fill2(
                Key::CLI_RESUMING.text(lang),
                human(left),
                human(j.total_size)
            )
        );
    }
    say!(
        "{}",
        i18n::fill1(Key::CLI_AVAILABLE_LINE.text(lang), human(plan.free))
    );
    if plan.keep_archive {
        say!(
            "{}",
            i18n::fill2(
                Key::CLI_REQUIRED_LINE.text(lang),
                human(plan.needed),
                human(plan.keep_free)
            )
        );
        say!("{DIM}{}{DIM:#}", Key::CLI_NOTE_KEPT.text(lang));
    } else {
        say!(
            "{}",
            i18n::fill1(Key::CLI_STANDARD_NEEDS.text(lang), human(plan.normal_needs))
        );
        say!(
            "{}",
            i18n::fill2(
                Key::CLI_REQUIRED_LINE.text(lang),
                human(plan.needed),
                human(plan.keep_free)
            )
        );
        if !plan.reclaims {
            say!("{DIM}{}{DIM:#}", Key::CLI_NOTE_OTHER_DRIVE.text(lang));
        }
        if plan.archive_kept_whole {
            say!("{DIM}{}{DIM:#}", Key::CLI_NOTE_KEPT_WHOLE.text(lang));
        }
    }
    if plan.fits {
        say!(
            "{}",
            i18n::fill1(Key::CLI_AVAILABLE_AFTER.text(lang), human(plan.free_after))
        );
        if !plan.keep_archive && plan.either && plan.normal_fits() {
            say!("{DIM}{}{DIM:#}", Key::CLI_NOTE_KEEP_ENOUGH.text(lang));
        }
    } else {
        let short = human(plan.needed + plan.keep_free - plan.free);
        println!(
            "{ERROR}{}{ERROR:#}",
            i18n::fill1(Key::CLI_INSUFFICIENT.text(lang), short)
        );
        if plan.keep_archive && plan.either && plan.in_place_fits() {
            println!(
                "{}",
                i18n::fill1(Key::CLI_WITHOUT_KEEP.text(lang), human(plan.in_place_needs))
            );
        }
        if plan.solid_block_bound {
            println!("{DIM}{}{DIM:#}", Key::CLI_SOLID_NOTE.text(lang));
        }
    }
    Ok(plan.fits)
}

fn check_all(target: Target, keep: bool, space: Space) -> Result<u8> {
    let archives = distinct(&target.archives)?;
    let code = each(&archives, false, |a| {
        check(
            a,
            target.output.clone(),
            target.secret.clone(),
            keep,
            &space,
        )
    });
    if archives.len() > 1 {
        say!();
        say!(
            "{DIM}{}{DIM:#}",
            Key::CLI_CHECK_MULTI_NOTE.text(i18n::current())
        );
    }
    Ok(code)
}

fn check(
    archive: &Path,
    output: Option<PathBuf>,
    secret: Secret,
    keep: bool,
    space: &Space,
) -> Result<u8> {
    let (state, _) = open(archive, output, false, secret)?;
    let fits = match state {
        State::Fresh(job, out) => {
            in_place_possible(&job, keep)?;
            report(&job, &out, None, space.keep_free, keep)?
        }
        State::Resume(job, journal) => {
            let keep = session::keeps_archive(&journal, keep)?;
            report(&job, &journal.output, Some(&journal), space.keep_free, keep)?
        }
        State::Orphan(_) => {
            say!("{}", Key::CLI_ORPHAN_CHECK.text(i18n::current()));
            true
        }
    };
    Ok(if fits { 0 } else { EXIT_NO_SPACE })
}

/// Refuses, before anything is touched, an archive that can only be
/// extracted while it is kept, such as a solid RAR, unless it is.
fn in_place_possible(job: &Job, keep: bool) -> Result<()> {
    match job.in_place_refusal() {
        Some(why) if !keep => bail!(
            "{}",
            i18n::fill1(Key::CLI_REFUSAL_KEEP.text(i18n::current()), why)
        ),
        _ => Ok(()),
    }
}

fn list(archive: &Path, secret: Secret) -> Result<u8> {
    let job = match open(archive, None, false, secret)?.0 {
        State::Fresh(job, _) | State::Resume(job, _) => job,
        State::Orphan(_) => bail!("no entries remain in this archive"),
    };
    println!(
        "{BOLD}{}{BOLD:#}",
        Key::CLI_LIST_HEADER.text(i18n::current())
    );
    for item in job.units.iter().rev().flat_map(|u| &u.items) {
        let path = item.path.display();
        match &item.kind {
            Kind::Dir => println!("{:>10}  {path}/", ""),
            Kind::File => println!("{:>10}  {path}", human(item.size)),
            Kind::Symlink => println!(
                "{:>10}  {path} {DIM}{}{DIM:#}",
                "",
                Key::CLI_LIST_SYMLINK.text(i18n::current())
            ),
            Kind::HardLink(t) => {
                println!(
                    "{:>10}  {path} {DIM}{}{DIM:#}",
                    "",
                    i18n::fill1(Key::CLI_LIST_LINK.text(i18n::current()), t.display())
                )
            }
        }
    }
    say!(
        "{DIM}{}{DIM:#}",
        i18n::fill4(
            Key::CLI_LIST_FOOTER.text(i18n::current()),
            entries(job.item_count()),
            human(job.remaining_size()),
            human(job.storage.original_len()),
            job.format_name
        )
    );
    Ok(0)
}

fn test_command(archive: &Path, secret: Secret) -> Result<u8> {
    let job = match open(archive, None, false, secret)?.0 {
        State::Fresh(job, _) => job,
        _ => bail!("{}", Key::CLI_PARTIAL_TEST.text(i18n::current())),
    };
    if let Err(e) = verify(&job) {
        eprintln!("{ERROR}error:{ERROR:#} {e:#}");
        return Ok(EXIT_DAMAGED);
    }
    say!(
        "{GOOD}{}{GOOD:#}",
        i18n::fill1(
            Key::CLI_TEST_PASSED.text(i18n::current()),
            entries(job.item_count())
        )
    );
    Ok(0)
}

fn entries(n: usize) -> String {
    let lang = i18n::current();
    if n == 1 {
        Key::CLI_ENTRIES_ONE.text(lang).to_string()
    } else {
        i18n::fill1(Key::CLI_ENTRIES_MANY.text(lang), n)
    }
}

fn restore(archive: &Path) -> Result<u8> {
    let r = hatchzip_core::restore::restore(archive)?;
    let name = r.archive.file_name().unwrap_or_default().to_string_lossy();
    if r.kept {
        say!(
            "{GOOD}{}{GOOD:#}",
            i18n::fill1(Key::CLI_RESTORE_KEPT.text(i18n::current()), &name)
        );
    } else {
        say!(
            "{GOOD}{}{GOOD:#}",
            i18n::fill3(
                Key::CLI_RESTORE_DONE.text(i18n::current()),
                &name,
                r.format,
                entries(r.entries)
            )
        );
    }
    say!(
        "{}",
        i18n::fill1(
            Key::CLI_FILES_REMAIN.text(i18n::current()),
            r.output.display()
        )
    );
    Ok(0)
}

/// How one archive of a run ended, for the summary of several.
enum Ended {
    Extracted,
    /// Stopped with the given status; `saved` if a journal holds progress.
    Stopped {
        code: u8,
        saved: bool,
    },
}

struct Options {
    keep: bool,
    verify_first: bool,
    keep_free: u64,
    yes: bool,
    /// Several archives: the summary says how to continue, not each one.
    several: bool,
}

fn extract_all(
    target: Target,
    keep: bool,
    verify_first: bool,
    space: Space,
    yes: bool,
) -> Result<u8> {
    let archives = distinct(&target.archives)?;
    let opts = Options {
        keep,
        verify_first,
        keep_free: space.keep_free,
        yes,
        several: archives.len() > 1,
    };
    // A mistyped name is cheap to catch before the first archive is
    // removed, rather than halfway through.
    if opts.several {
        for a in &archives {
            // A run that ended just before its cleanup has a journal left.
            let (primary, paths) = locate(a)?;
            if !primary.exists() && !paths.exists() {
                bail!(
                    "{}",
                    i18n::fill1(Key::CLI_NOT_FOUND.text(i18n::current()), a.display())
                );
            }
        }
    }
    // Installed once: a second handler can't be set in the same process.
    ctrlc::set_handler(|| {
        if STOP.swap(true, Ordering::Relaxed) {
            std::process::exit(EXIT_PAUSED.into());
        }
        eprintln!("\n{}", Key::CLI_PAUSE_NOTE.text(i18n::current()));
    })
    .context("installing Ctrl+C handler")?;

    let mut done = 0;
    for (i, archive) in archives.iter().enumerate() {
        if opts.several {
            if i > 0 {
                say!();
            }
            say!(
                "{BOLD}{}{BOLD:#} {}",
                i18n::fill2(
                    Key::CLI_ARCHIVE_N_OF.text(i18n::current()),
                    i + 1,
                    archives.len()
                ),
                archive.display()
            );
        }
        let ended = match extract(archive, target.output.clone(), target.secret.clone(), &opts) {
            Ok(e) => e,
            Err(e) => Ended::Stopped {
                code: fail(&e),
                saved: locate(archive).is_ok_and(|(_, p)| p.exists()),
            },
        };
        match ended {
            Ended::Extracted => done += 1,
            Ended::Stopped { code, saved } => {
                if opts.several {
                    summary(&archives, done, code, saved);
                }
                return Ok(code);
            }
        }
    }
    if opts.several {
        say!();
        say!(
            "{GOOD}{}{GOOD:#}",
            i18n::fill1(Key::CLI_ALL_EXTRACTED.text(i18n::current()), archives.len())
        );
    }
    Ok(0)
}

/// What a run of several archives did, after `archives[done]` stopped it.
/// Shown even under `--quiet`, since it is the answer to what happened.
fn summary(archives: &[PathBuf], done: usize, code: u8, saved: bool) {
    let lang = i18n::current();
    let stopped = if code == EXIT_PAUSED {
        Key::CLI_STATE_PAUSED.text(lang)
    } else {
        Key::CLI_STATE_NOT_EXTRACTED.text(lang)
    };
    println!();
    println!(
        "{ERROR}{}{ERROR:#}",
        i18n::fill2(Key::CLI_STOPPED.text(lang), done + 1, archives.len())
    );
    for (i, a) in archives.iter().enumerate() {
        let state = match i.cmp(&done) {
            std::cmp::Ordering::Less => Key::CLI_STATE_EXTRACTED.text(lang).to_string(),
            std::cmp::Ordering::Equal if saved => {
                i18n::fill1(Key::CLI_STATE_SAVED.text(lang), stopped)
            }
            std::cmp::Ordering::Equal => stopped.to_string(),
            std::cmp::Ordering::Greater => Key::CLI_STATE_NOT_STARTED.text(lang).to_string(),
        };
        println!("  {}  {DIM}({state}){DIM:#}", a.display());
    }
    println!("{}", Key::CLI_CONTINUE.text(lang));
}

fn extract(
    archive: &Path,
    output: Option<PathBuf>,
    secret: Secret,
    opts: &Options,
) -> Result<Ended> {
    let keep_free = opts.keep_free;
    let no_space = Ok(Ended::Stopped {
        code: EXIT_NO_SPACE,
        saved: false,
    });
    // Refused before the writable open, which would finish a truncation
    // the last run left pending.
    if opts.keep {
        let (_, paths) = locate(archive)?;
        if paths.exists() {
            session::keeps_archive(&Journal::load(&paths)?.0, true)?;
        }
    }
    let (state, paths) = open(archive, output, true, secret)?;
    let (job, journal) = match state {
        State::Orphan(journal) => {
            engine::finish_orphaned(&journal, &paths)?;
            say!(
                "{GOOD}{}{GOOD:#}",
                i18n::fill1(
                    Key::CLI_ORPHAN_DONE.text(i18n::current()),
                    journal.output.display()
                )
            );
            return Ok(Ended::Extracted);
        }
        State::Resume(job, journal) => {
            let keep = session::keeps_archive(&journal, opts.keep)?;
            if keep && !opts.keep {
                say!("{}", Key::CLI_RESUME_KEEP_NOTE.text(i18n::current()));
            }
            if !report(&job, &journal.output, Some(&journal), keep_free, keep)? {
                return Ok(Ended::Stopped {
                    code: EXIT_NO_SPACE,
                    saved: true,
                });
            }
            session::clear_uncommitted(&job, &journal.output)?;
            (job, journal)
        }
        State::Fresh(job, out) => {
            in_place_possible(&job, opts.keep)?;
            if !report(&job, &out, None, keep_free, opts.keep)? {
                return no_space;
            }
            if opts.verify_first
                && let Err(e) = verify(&job)
            {
                eprintln!("{ERROR}error:{ERROR:#} {e:#}");
                return Ok(Ended::Stopped {
                    code: EXIT_DAMAGED,
                    saved: false,
                });
            }
            // Nothing is removed when the archive is kept, so there is
            // nothing to confirm.
            if !opts.yes && !opts.keep {
                confirm(&display_name(&job), &out)?;
            }
            let journal = session::begin(&job, &paths, out, opts.keep)?;
            (job, journal)
        }
    };

    let keep = journal.keep;
    let bar = progress_bar(journal.total_size, Key::CLI_EXTRACTING_BAR);
    bar.set_position(journal.total_size - job.remaining_size());
    if !keep {
        bar.set_message(i18n::fill1(
            Key::CLI_ARCHIVE_PROGRESS.text(i18n::current()),
            human(journal.committed_len),
        ));
    }

    let output = journal.output.clone();
    let count = job.item_count();
    let (b1, b2) = (bar.clone(), bar.clone());
    let run = Run {
        job,
        journal,
        paths,
        stop: &STOP,
        progress: Progress {
            on_bytes: Box::new(move |n| b1.inc(n)),
            on_commit: Box::new(move |len| {
                b2.set_message(i18n::fill1(
                    Key::CLI_ARCHIVE_PROGRESS.text(i18n::current()),
                    human(len),
                ))
            }),
        },
        keep_free,
    };
    let outcome = run.execute();
    bar.finish_and_clear();

    match outcome {
        Ok(Outcome::Finished) => {
            let archive = if keep {
                Key::CLI_EXTRACT_KEPT.text(i18n::current())
            } else {
                Key::CLI_EXTRACT_REMOVED.text(i18n::current())
            };
            say!(
                "{GOOD}{}{GOOD:#}",
                i18n::fill3(
                    Key::CLI_EXTRACT_DONE.text(i18n::current()),
                    entries(count),
                    output.display(),
                    archive
                )
            );
            Ok(Ended::Extracted)
        }
        Ok(Outcome::Paused) => {
            if !opts.several {
                say!("{}", Key::CLI_PAUSED.text(i18n::current()));
            }
            Ok(Ended::Stopped {
                code: EXIT_PAUSED,
                saved: true,
            })
        }
        Err(e) => {
            eprintln!("{ERROR}error:{ERROR:#} {e:#}");
            if !opts.several {
                eprintln!("{}", Key::CLI_PROGRESS_SAVED.text(i18n::current()));
            }
            // A file whose password differs from the one proven at the
            // start, possible in RAR 4, is still a password problem.
            let wrong = e.chain().any(|c| c.is::<WrongPassword>());
            Ok(Ended::Stopped {
                code: if wrong { EXIT_PASSWORD } else { EXIT_FAILURE },
                saved: true,
            })
        }
    }
}

fn verify(job: &Job) -> Result<()> {
    let bar = progress_bar(job.remaining_size(), Key::CLI_VERIFYING_BAR);
    let b = bar.clone();
    let result = job.verify(&move |n| b.inc(n));
    bar.finish_and_clear();
    result.context(Key::CLI_DAMAGED.text(i18n::current()))
}

fn confirm(archive: &str, output: &Path) -> Result<()> {
    if !io::stdin().is_terminal() {
        bail!("{}", Key::CLI_YES_REQUIRED.text(i18n::current()));
    }
    print!(
        "{}",
        i18n::fill2(
            Key::CLI_CONFIRM.text(i18n::current()),
            output.display(),
            archive
        )
    );
    io::stdout().flush()?;
    let mut answer = String::new();
    io::stdin().lock().read_line(&mut answer)?;
    if !matches!(answer.trim(), "y" | "Y" | "yes") {
        bail!("{}", Key::CLI_CANCELLED.text(i18n::current()));
    }
    Ok(())
}

fn progress_bar(total: u64, prefix: Key) -> ProgressBar {
    if QUIET.load(Ordering::Relaxed) {
        return ProgressBar::hidden();
    }
    let bar = ProgressBar::new(total);
    bar.set_style(
        ProgressStyle::with_template(
            "{prefix} [{bar:32}] {binary_bytes}/{binary_total_bytes}  {binary_bytes_per_sec}  eta {eta}  {msg}",
        )
        .expect("valid template")
        .progress_chars("=> "),
    );
    bar.set_prefix(prefix.text(i18n::current()));
    bar.enable_steady_tick(Duration::from_millis(200));
    bar
}
