//! The HatchZip desktop app. The binary in `main.rs` only parses arguments
//! and picks a window; the windows are built here so the integration test
//! can drive them without a display.

pub mod args;
pub mod backend;
mod entries;
pub mod main_window;
mod password;
mod platform;
mod runner;
pub mod settings;
pub mod shell;
mod strings;
mod task;
pub mod task_window;
mod text;
mod work;

slint::include_modules!();

/// The desktop file name packaging installs, which Wayland and X11 use to
/// match the window to its launcher entry and icon.
pub const APP_ID: &str = "io.github.xevrion.hatchzip";
