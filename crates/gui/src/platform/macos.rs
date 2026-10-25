use std::path::PathBuf;

use hatchzip_i18n::{Key, Lang};

/// The settings switch, named after this system's file manager.
pub fn shell_menu_label(lang: Lang) -> &'static str {
    Key::SHELL_MENU_LABEL_FINDER.text(lang)
}

/// hatchzip-shell can't add Finder entries yet, so the setting is hidden.
pub const SHELL_MENU: bool = false;

/// `~/Library/Application Support/HatchZip`.
pub fn config_dir() -> Option<PathBuf> {
    Some(std::env::home_dir()?.join("Library/Application Support/HatchZip"))
}
