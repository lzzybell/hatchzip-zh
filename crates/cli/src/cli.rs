//! Command line definition. Also the source of the shell completions and the
//! man page, so the three never disagree. The wording lives in the catalog,
//! so `hatchzip --help` reads in the language in use.

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use clap_complete::Shell;
use hatchzip_i18n::{self as i18n, Key, Lang};

pub const EXIT_FAILURE: u8 = 1;
pub const EXIT_NO_SPACE: u8 = 3;
pub const EXIT_DAMAGED: u8 = 4;
pub const EXIT_PASSWORD: u8 = 5;
pub const EXIT_PAUSED: u8 = 130;

#[derive(Parser)]
#[command(
    name = "hatchzip",
    version,
    propagate_version = true,
    about = "",
    long_about = ""
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Quiet.
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Extract.
    #[command(visible_alias = "x")]
    Extract {
        #[command(flatten)]
        target: Target,
        #[command(flatten)]
        keep: Keep,
        /// Verify.
        #[arg(long)]
        verify_first: bool,
        #[command(flatten)]
        space: Space,
        /// Yes.
        #[arg(short, long)]
        yes: bool,
    },
    /// Check.
    #[command(visible_alias = "c")]
    Check {
        #[command(flatten)]
        target: Target,
        #[command(flatten)]
        keep: Keep,
        #[command(flatten)]
        space: Space,
    },
    /// List.
    #[command(visible_alias = "l")]
    List {
        /// Archives.
        #[arg(required = true, value_name = "ARCHIVE")]
        archives: Vec<PathBuf>,
        #[command(flatten)]
        secret: Secret,
    },
    /// Test.
    #[command(visible_alias = "t")]
    Test {
        /// Archives.
        #[arg(required = true, value_name = "ARCHIVE")]
        archives: Vec<PathBuf>,
        #[command(flatten)]
        secret: Secret,
    },
    /// Restore.
    Restore {
        /// Archive.
        archive: PathBuf,
    },
    /// Completions.
    Completions { shell: Shell },
    /// Man page.
    Man,
}

#[derive(Args)]
pub struct Target {
    /// Archives.
    #[arg(required = true, value_name = "ARCHIVE")]
    pub archives: Vec<PathBuf>,
    /// Output.
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,
    #[command(flatten)]
    pub secret: Secret,
}

#[derive(Args)]
pub struct Keep {
    /// Keep the archive.
    #[arg(long)]
    pub keep_archive: bool,
}

#[derive(Args, Clone)]
pub struct Secret {
    /// Password.
    #[arg(
        short,
        long,
        env = "HATCHZIP_PASSWORD",
        hide_env_values = true,
        value_name = "PASSWORD"
    )]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct Space {
    /// Reserved space.
    #[arg(long, value_name = "SIZE", default_value = "2GB", value_parser = parse_size)]
    pub keep_free: u64,
}

/// The long about, ending with what this build reads.
fn long_about(lang: Lang) -> String {
    let mut text = Key::CLI_LONG_ABOUT_BASE.text(lang).to_string();
    text.push_str(if hatchzip_core::archive::RAR_SUPPORTED {
        Key::CLI_LONG_ABOUT_RAR_TAIL.text(lang)
    } else {
        Key::CLI_LONG_ABOUT_NO_RAR_TAIL.text(lang)
    });
    text
}

/// Every help text, in the language in use.
pub fn localized() -> clap::Command {
    let lang = i18n::current();
    let t = |k: Key| k.text(lang);
    let mut cmd = Cli::command()
        .about(t(Key::CLI_ABOUT))
        .long_about(long_about(lang))
        .after_long_help(t(Key::CLI_AFTER_LONG_HELP));

    // Clap's own flags.
    cmd = set_help(cmd, "quiet", Key::CLI_QUIET, lang);
    cmd = set_help(cmd, "help", Key::CLI_HELP_FLAG, lang);
    cmd = set_help(cmd, "version", Key::CLI_VERSION_FLAG, lang);

    let extract = t(Key::CLI_EXTRACT_LONG).to_string();
    cmd = cmd.mut_subcommand("extract", move |s| {
        let s = s
            .about(t(Key::CLI_EXTRACT_ABOUT))
            .long_about(extract.clone());
        let s = set_help(s, "archives", Key::CLI_ARCHIVE_ARG, lang);
        let s = set_help(s, "output", Key::CLI_OUTPUT, lang);
        let s = set_help(s, "password", Key::CLI_PASSWORD, lang);
        let s = set_help(s, "keep_archive", Key::CLI_KEEP_ARCHIVE, lang);
        let s = set_help(s, "verify_first", Key::CLI_VERIFY_FIRST, lang);
        let s = set_help(s, "keep_free", Key::CLI_KEEP_FREE, lang);
        let s = set_help(s, "yes", Key::CLI_YES, lang);
        set_help(s, "help", Key::CLI_HELP_FLAG, lang)
    });

    cmd = cmd.mut_subcommand("check", move |s| {
        let s = s.about(t(Key::CLI_CHECK_ABOUT));
        let s = set_help(s, "archives", Key::CLI_ARCHIVE_ARG, lang);
        let s = set_help(s, "output", Key::CLI_OUTPUT, lang);
        let s = set_help(s, "password", Key::CLI_PASSWORD, lang);
        let s = set_help(s, "keep_archive", Key::CLI_KEEP_ARCHIVE, lang);
        let s = set_help(s, "keep_free", Key::CLI_KEEP_FREE, lang);
        set_help(s, "help", Key::CLI_HELP_FLAG, lang)
    });

    for name in ["list", "test"] {
        let about = if name == "list" {
            Key::CLI_LIST_ABOUT
        } else {
            Key::CLI_TEST_ABOUT
        };
        cmd = cmd.mut_subcommand(name, move |s| {
            let s = s.about(t(about));
            let s = set_help(s, "archives", Key::CLI_ARCHIVE_ARG, lang);
            let s = set_help(s, "password", Key::CLI_PASSWORD, lang);
            set_help(s, "help", Key::CLI_HELP_FLAG, lang)
        });
    }

    let restore = t(Key::CLI_RESTORE_LONG).to_string();
    cmd = cmd.mut_subcommand("restore", move |s| {
        let s = s
            .about(t(Key::CLI_RESTORE_ABOUT))
            .long_about(restore.clone());
        let s = set_help(s, "archive", Key::CLI_RESTORE_ARCHIVE_ARG, lang);
        set_help(s, "help", Key::CLI_HELP_FLAG, lang)
    });

    let completions = t(Key::CLI_COMPLETIONS_LONG).to_string();
    cmd = cmd.mut_subcommand("completions", move |s| {
        let s = s
            .about(t(Key::CLI_COMPLETIONS_ABOUT))
            .long_about(completions.clone());
        let s = set_help(s, "shell", Key::CLI_SHELL_ARG, lang);
        set_help(s, "help", Key::CLI_HELP_FLAG, lang)
    });

    let man = t(Key::CLI_MAN_LONG).to_string();
    cmd.mut_subcommand("man", move |s| {
        let s = s.about(t(Key::CLI_MAN_ABOUT)).long_about(man.clone());
        set_help(s, "help", Key::CLI_HELP_FLAG, lang)
    })
}

/// Replaces an argument's help, where the argument exists on this command.
/// `mut_arg` panics on an unknown id, and not every subcommand takes every
/// argument.
fn set_help(cmd: clap::Command, id: &str, key: Key, lang: Lang) -> clap::Command {
    if !cmd.get_arguments().any(|a| a.get_id().as_str() == id) {
        return cmd;
    }
    let text = key.text(lang);
    cmd.mut_arg(id, |a| a.help(text).long_help(text))
}

/// Prints the help in the language in use and says whether it did, so clap's
/// own headings ("Usage:", "Commands:") never show through.
pub fn print_help_if_asked(cmd: &clap::Command, argv: &[OsString]) -> bool {
    let mut wanted = false;
    let mut long = false;
    for arg in argv.iter().skip(1) {
        let text = arg.to_string_lossy();
        if text == "--" {
            break;
        }
        if text == "--help" {
            wanted = true;
            long = true;
        } else if text == "-h" {
            wanted = true;
        }
    }
    if !wanted {
        return false;
    }
    let sub = argv
        .iter()
        .skip(1)
        .map(|a| a.to_string_lossy().into_owned())
        .find(|a| !a.starts_with('-'))
        .and_then(|name| cmd.find_subcommand(&name).cloned());
    let mut target = match sub {
        Some(s) => {
            let name = s.get_name().to_string();
            s.bin_name(format!("hatchzip {name}"))
        }
        None => cmd.clone(),
    }
    .color(clap::ColorChoice::Never);
    let rendered = if long {
        target.render_long_help()
    } else {
        target.render_help()
    };
    print!("{}", headings(&rendered.to_string()));
    true
}

/// Clap's own section headings and its built-in flag texts, in Chinese.
fn headings(text: &str) -> String {
    if i18n::current() != Lang::Zh {
        return text.to_string();
    }
    text.replace("Usage:", "用法:")
        .replace("Commands:", "命令:")
        .replace("Options:", "选项:")
        .replace("Arguments:", "参数:")
        .replace("[alias: ", "[别名: ")
        .replace(
            "Print help (see a summary with '-h')",
            "显示帮助(用 '-h' 查看简要说明)",
        )
        .replace(
            "Print help (see more with '--help')",
            "显示帮助(用 '--help' 查看更多)",
        )
        .replace("Print help", "显示帮助")
        .replace("Print version", "显示版本")
        .replace(
            "Print this message or the help of the given subcommand(s)",
            "显示此信息或给定子命令的帮助",
        )
}

/// Sizes like `500MB`, `2GB` or `1.5TB`, in powers of 1000 to match what
/// HatchZip prints.
pub fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let split = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(s.len());
    let (num, unit) = s.split_at(split);
    let lang = i18n::current();
    let num: f64 = num
        .parse()
        .map_err(|_| i18n::fill1(Key::CLI_INVALID_SIZE.text(lang), s))?;
    let mult = match unit.trim().to_ascii_uppercase().as_str() {
        "" | "B" => 1.0,
        "K" | "KB" => 1e3,
        "M" | "MB" => 1e6,
        "G" | "GB" => 1e9,
        "T" | "TB" => 1e12,
        _ => {
            return Err(i18n::fill1(Key::CLI_UNKNOWN_UNIT.text(lang), s));
        }
    };
    Ok((num * mult) as u64)
}

/// Parses the command line against the localized command, so the help and
/// every message are in the language in use.
pub fn parse() -> Cli {
    let argv: Vec<OsString> = std::env::args_os().collect();
    let cmd = localized();
    let matches = cmd.get_matches_from(argv);
    Cli::from_arg_matches(&matches).expect("clap built this command line")
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn command_line_is_well_formed() {
        i18n::set_current(Lang::En);
        Cli::command().debug_assert();
        localized().debug_assert();
    }

    #[test]
    fn sizes_parse() {
        i18n::set_current(Lang::En);
        assert_eq!(parse_size("0"), Ok(0));
        assert_eq!(parse_size("500MB"), Ok(500_000_000));
        assert_eq!(parse_size("2gb"), Ok(2_000_000_000));
        assert_eq!(parse_size("1.5 TB"), Ok(1_500_000_000_000));
        assert!(parse_size("ten").is_err());
        assert!(parse_size("5PB").is_err());
    }
}
