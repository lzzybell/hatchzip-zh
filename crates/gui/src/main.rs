// A GUI app on Windows must not open a console window next to itself. This
// attribute can only live at the crate root, so it is the one `cfg(windows)`
// outside a platform module. Debug builds keep the console for their output.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::process::ExitCode;
use std::rc::Rc;

use anyhow::Result;
use clap::Parser;

use hatchzip_gui::args::{Args, Launch};
use hatchzip_gui::{APP_ID, backend, main_window, shell, task_window};
use hatchzip_i18n::{self as i18n, Key, Lang};

fn main() -> ExitCode {
    match run(Args::parse().launch()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("hatchzip-gui: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(launch: Launch) -> Result<()> {
    // The right-click entries are registered from here too, and carry the
    // labels in the chosen language.
    let lang = hatchzip_gui::settings::Settings::saved()
        .lang
        .as_deref()
        .and_then(Lang::parse)
        .unwrap_or_else(i18n::current);
    i18n::set_current(lang);
    match launch {
        Launch::RegisterShell => {
            for place in shell::register()? {
                println!("{place}");
            }
            return Ok(());
        }
        Launch::UnregisterShell => {
            shell::unregister()?;
            println!("{}", Key::NOTICE_MENU_REMOVED.text(i18n::current()));
            return Ok(());
        }
        _ => {}
    }
    let drops = Rc::new(backend::Drops::default());
    backend::select(drops.clone())?;
    slint::set_xdg_app_id(APP_ID)?;
    match launch {
        Launch::Main(archive) => main_window::run(archive, drops),
        Launch::Task(task) => task_window::run(task),
        Launch::RegisterShell | Launch::UnregisterShell => unreachable!("handled above"),
    }
}
