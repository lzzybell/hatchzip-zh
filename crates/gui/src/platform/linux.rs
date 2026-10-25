use std::path::PathBuf;

use hatchzip_i18n::{Key, Lang};

/// The settings switch, named after this system's file manager.
pub fn shell_menu_label(lang: Lang) -> &'static str {
    Key::SHELL_MENU_LABEL.text(lang)
}

/// The file managers hatchzip-shell knows (Dolphin, GNOME Files, Nemo).
pub const SHELL_MENU: bool = true;

/// `$XDG_CONFIG_HOME/hatchzip`, or `~/.config/hatchzip`.
pub fn config_dir() -> Option<PathBuf> {
    let base = match std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        Some(d) => PathBuf::from(d),
        None => std::env::home_dir()?.join(".config"),
    };
    Some(base.join("hatchzip"))
}
