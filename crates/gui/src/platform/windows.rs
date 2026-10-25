use std::path::PathBuf;

use hatchzip_i18n::{Key, Lang};

/// The settings switch, named after this system's file manager.
pub fn shell_menu_label(lang: Lang) -> &'static str {
    Key::SHELL_MENU_LABEL_EXPLORER.text(lang)
}

/// Explorer, through the per-user registry hive.
pub const SHELL_MENU: bool = true;

/// `%APPDATA%\HatchZip`, which roams with the user's profile.
pub fn config_dir() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA").filter(|v| !v.is_empty())?;
    Some(PathBuf::from(appdata).join("HatchZip"))
}
