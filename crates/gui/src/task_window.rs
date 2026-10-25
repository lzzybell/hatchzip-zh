//! The small window behind "Extract here", "Extract to a folder" and "Test".
//! The right-click was the confirmation, so it starts as soon as it shows,
//! unless the space check says no. Several archives run one after another,
//! each checked just before it starts; one that doesn't finish stops the
//! rest until it is resumed.

use std::path::{Path, PathBuf};

use anyhow::Result;
use hatchzip_core::session;
use hatchzip_i18n::{self as i18n, Lang};
use slint::{CloseRequestResponse, ComponentHandle};

use crate::TaskWindow;
use crate::args::{TaskArgs, TaskKind};
use crate::runner::Runner;
use crate::settings::Settings;
use crate::task::Task;
use crate::work::Request;

/// Where each kind of task puts the files. `None` for a test.
pub fn output_for(kind: TaskKind, primary: &Path) -> Option<PathBuf> {
    match kind {
        TaskKind::ExtractHere => Some(primary.parent().unwrap_or(Path::new(".")).to_path_buf()),
        TaskKind::ExtractTo => Some(session::default_output(primary)),
        TaskKind::Test => None,
    }
}

pub fn run(args: TaskArgs) -> Result<()> {
    let (ui, start) = build(args, Settings::saved())?;
    crate::backend::when_drawn(ui.window(), start);
    ui.run()?;
    Ok(())
}

/// The task and request for one archive. The main window's reserved space
/// and verification settings apply; whether the archive is kept is the
/// menu entry's choice.
fn job_for(
    kind: TaskKind,
    archive: &Path,
    keep_archive: bool,
    settings: &Settings,
) -> (Task, Request) {
    // Only resolves the path; a missing file is reported by the run itself.
    let primary = session::locate(archive).map_or_else(|_| archive.to_path_buf(), |(p, _)| p);
    let name = primary
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let archive = archive.to_path_buf();
    match output_for(kind, &primary) {
        Some(out) => (
            Task::extract(&name, Some(&out), keep_archive),
            Request::Extract {
                archive,
                output: Some(out),
                keep_free: settings.keep_free,
                verify_first: settings.verify_first,
                keep_archive,
                password: None,
            },
        ),
        None => (
            Task::test(&name),
            Request::Test {
                archive,
                password: None,
            },
        ),
    }
}

/// The window, shown, and the function that starts the task. Nothing
/// touches an archive until that is called.
pub fn build(args: TaskArgs, settings: Settings) -> Result<(TaskWindow, impl FnOnce() + 'static)> {
    let lang = settings
        .lang
        .as_deref()
        .and_then(Lang::parse)
        .unwrap_or_else(i18n::current);
    i18n::set_current(lang);
    let ui = TaskWindow::new()?;
    crate::strings::apply_task(&ui, lang);
    let jobs: Vec<(Task, Request)> = args
        .archives
        .iter()
        .map(|a| job_for(args.kind, a, args.keep_archive, &settings))
        .collect();
    let Some((task, _)) = jobs.first() else {
        anyhow::bail!("no archive given");
    };

    let weak = ui.as_weak();
    let runner = Runner::install(move |task: &Task| {
        if let Some(ui) = weak.upgrade() {
            ui.set_task(task.view.clone());
            ui.set_plan(task.plan.clone());
        }
    });
    ui.set_task(task.view.clone());

    let r = runner.clone();
    ui.on_pause(move || r.pause());
    let r = runner.clone();
    ui.on_resume(move || r.retry());
    let r = runner.clone();
    ui.on_open_folder(move || {
        let task = r.task.borrow();
        if let Some(out) = task.as_ref().and_then(|t| t.output.clone()) {
            let _ = open::that_detached(out);
        }
    });
    let r = runner.clone();
    ui.on_give_back(move || r.give_back());
    crate::password::host_in!(ui);
    let r = runner.clone();
    ui.on_dismiss(move || {
        if r.may_close() {
            let _ = slint::quit_event_loop();
        }
    });
    let r = runner.clone();
    ui.window().on_close_requested(move || {
        if r.may_close() {
            CloseRequestResponse::HideWindow
        } else {
            CloseRequestResponse::KeepWindowShown
        }
    });

    ui.show()?;
    Ok((ui, move || runner.start_queue(jobs)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_task_writes_where_its_menu_entry_says() {
        let primary = Path::new("/home/me/Downloads/game.7z.001");
        assert_eq!(
            output_for(TaskKind::ExtractHere, primary),
            Some(PathBuf::from("/home/me/Downloads"))
        );
        assert_eq!(
            output_for(TaskKind::ExtractTo, primary),
            Some(PathBuf::from("/home/me/Downloads/game"))
        );
        assert_eq!(output_for(TaskKind::Test, primary), None);
    }
}
