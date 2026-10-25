//! Puts HatchZip in the file manager's right-click menu, per user, without
//! admin rights. Every entry runs the GUI with one of the command line flags
//! in [`ACTIONS`], which the GUI treats as already confirmed.
//!
//! Registration only ever writes entries under names HatchZip owns, and
//! [`unregister`] removes exactly those.

mod platform;

use std::path::{Path, PathBuf};

use anyhow::Result;
use hatchzip_i18n::{Key, Lang};

pub struct Action {
    /// Stable identifier, used in file and registry key names.
    pub id: &'static str,
    /// The label, in every language.
    pub key: Key,
    /// The GUI's flags, put before the archive paths.
    pub flag: &'static str,
}

impl Action {
    pub fn label(&self, lang: Lang) -> &'static str {
        self.key.text(lang)
    }
}

pub const ACTIONS: &[Action] = &[
    Action {
        id: "extract-here",
        key: Key::SHELL_EXTRACT_HERE,
        flag: "--extract-here",
    },
    Action {
        id: "extract-here-keep",
        key: Key::SHELL_EXTRACT_HERE_KEEP,
        flag: "--keep-archive --extract-here",
    },
    Action {
        id: "extract-to",
        key: Key::SHELL_EXTRACT_TO,
        flag: "--extract-to",
    },
    Action {
        id: "test",
        key: Key::SHELL_TEST,
        flag: "--test",
    },
];

/// File types the menu appears on. `.001` covers the first part of split
/// archives, which is the part HatchZip is opened with. Windows sees only the
/// last extension of `game.tar.gz`, so `.gz` is listed; a `.gz` that holds no
/// tar is refused before anything changes.
/// With RAR support, `.rar` and `.r00`, the second volume of an old-style
/// set; any volume opens the whole set.
#[cfg(not(feature = "rar"))]
pub const EXTENSIONS: &[&str] = &["zip", "7z", "tar", "tgz", "gz", "001"];
#[cfg(feature = "rar")]
pub const EXTENSIONS: &[&str] = &["zip", "7z", "tar", "tgz", "gz", "001", "rar", "r00"];

/// Every extension any build registers, so that unregistering from a build
/// without RAR support still removes what a build with it wrote.
pub const ALL_EXTENSIONS: &[&str] = &["zip", "7z", "tar", "tgz", "gz", "001", "rar", "r00"];

/// Where the per-user menu entries live. [`Dirs::current`] is the real
/// location; tests point it somewhere temporary.
#[derive(Clone, Debug)]
pub struct Dirs {
    /// `$XDG_DATA_HOME` on Linux. Unused on Windows, where entries go to the
    /// per-user registry hive.
    pub data: PathBuf,
}

impl Dirs {
    pub fn current() -> Result<Self> {
        platform::current_dirs()
    }
}

/// Adds the menu entries, pointing them at `gui`, the `hatchzip-gui`
/// executable. Safe to run again, for instance after an update moves it.
/// Returns a short description of each place an entry was written.
pub fn register(dirs: &Dirs, gui: &Path) -> Result<Vec<String>> {
    platform::register(dirs, gui)
}

/// Removes every entry [`register`] could have written.
pub fn unregister(dirs: &Dirs) -> Result<()> {
    platform::unregister(dirs)
}

pub fn is_registered(dirs: &Dirs) -> bool {
    platform::is_registered(dirs)
}
