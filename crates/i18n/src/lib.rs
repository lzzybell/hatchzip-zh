//! The wording HatchZip shows and which language it is shown in. The engine's
//! own error text stays in English: it is rare, technical, and carries names
//! and paths that would not survive translation.

mod keys;

pub use keys::Key;

use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

/// The language the windows and the command line speak.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Lang {
    #[default]
    En,
    Zh,
}

impl Lang {
    /// Both choices, in the order the menu lists them.
    pub const ALL: [Lang; 2] = [Lang::En, Lang::Zh];

    pub fn tag(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Zh => "zh",
        }
    }

    /// The name shown in the language menu, in that language itself.
    pub fn native(self) -> &'static str {
        match self {
            Lang::En => Key::LANG_ENGLISH.text(self),
            Lang::Zh => Key::LANG_CHINESE.text(self),
        }
    }

    /// `en`, `en-US`, `zh`, `zh-CN`, `zh_CN`. Anything else is not a choice.
    pub fn parse(tag: &str) -> Option<Lang> {
        let tag = tag.trim().to_ascii_lowercase();
        if tag.starts_with("zh") {
            Some(Lang::Zh)
        } else if tag.starts_with("en") {
            Some(Lang::En)
        } else {
            None
        }
    }

    /// HATCHZIP_LANG, else the language the app saved, else the system's own.
    pub fn detect() -> Lang {
        if let Some(lang) = std::env::var("HATCHZIP_LANG")
            .ok()
            .and_then(|v| Lang::parse(&v))
        {
            return lang;
        }
        saved_language().unwrap_or_else(system_language)
    }
}

/// The language the window saved, which the command line follows too. A
/// missing or malformed file is simply no answer.
fn saved_language() -> Option<Lang> {
    let text = std::fs::read_to_string(config_dir()?.join("settings.json")).ok()?;
    let after = text.split("\"lang\"").nth(1)?;
    let value = after.split(':').nth(1)?.trim_start();
    let value = value.strip_prefix('"')?;
    Lang::parse(value.split('"').next()?)
}

#[cfg(windows)]
fn config_dir() -> Option<std::path::PathBuf> {
    let appdata = std::env::var_os("APPDATA").filter(|v| !v.is_empty())?;
    Some(std::path::PathBuf::from(appdata).join("HatchZip"))
}

#[cfg(target_os = "macos")]
fn config_dir() -> Option<std::path::PathBuf> {
    Some(std::env::home_dir()?.join("Library/Application Support/HatchZip"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn config_dir() -> Option<std::path::PathBuf> {
    let base = match std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        Some(d) => std::path::PathBuf::from(d),
        None => std::env::home_dir()?.join(".config"),
    };
    Some(base.join("hatchzip"))
}

/// Unset until the first question, so starting up costs one system call.
static CURRENT: AtomicU8 = AtomicU8::new(u8::MAX);

/// The language in use. Fixed on first use, then changed only by [`set_current`].
pub fn current() -> Lang {
    match CURRENT.load(Ordering::Relaxed) {
        0 => Lang::En,
        1 => Lang::Zh,
        _ => {
            let lang = Lang::detect();
            set_current(lang);
            lang
        }
    }
}

pub fn set_current(lang: Lang) {
    CURRENT.store(
        match lang {
            Lang::En => 0,
            Lang::Zh => 1,
        },
        Ordering::Relaxed,
    );
}

/// The phrase `key` in the language in use.
pub fn text(key: Key) -> &'static str {
    key.text(current())
}

/// `1 entry`, `3 entries`, `3 个条目`.
pub fn entries(lang: Lang, n: usize) -> String {
    match lang {
        Lang::En => format!(
            "{n} {}",
            if n == 1 {
                Key::ENTRY_ONE.text(lang)
            } else {
                Key::ENTRY_MANY.text(lang)
            }
        ),
        Lang::Zh => format!("{n} {}", Key::ENTRY_MANY.text(lang)),
    }
}

/// A time remaining, rounded to the unit that matters.
pub fn duration(lang: Lang, d: Duration) -> String {
    let unit = |n: u64, one: Key, many: Key| {
        format!(
            "{n} {}",
            if n == 1 {
                one.text(lang)
            } else {
                many.text(lang)
            }
        )
    };
    let s = d.as_secs();
    match s {
        0..=4 => Key::LESS_THAN_FIVE_SECONDS.text(lang).to_string(),
        5..=59 => unit(s, Key::SECOND_ONE, Key::SECOND_MANY),
        60..=3599 => unit(s.div_ceil(60), Key::MINUTE_ONE, Key::MINUTE_MANY),
        _ => format!(
            "{} {}",
            unit(s / 3600, Key::HOUR_ONE, Key::HOUR_MANY),
            unit((s % 3600) / 60, Key::MINUTE_ONE, Key::MINUTE_MANY)
        ),
    }
}

/// "3.5 GB in 12 entries remaining" and "共 12 个条目,剩余 3.5 GB": the two
/// halves swap places, so this is a function rather than a template.
pub fn summary_remaining(lang: Lang, size: &str, entries: &str) -> String {
    match lang {
        Lang::En => format!("{size} in {entries} remaining"),
        Lang::Zh => format!("共 {entries},剩余 {size}"),
    }
}

/// `{}` placeholders in `template`, filled in order. A phrase can be a
/// `&'static str` chosen at run time, which `format!` cannot take.
fn fill_all(template: &str, args: &[&str]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    for arg in args {
        match rest.split_once("{}") {
            Some((before, after)) => {
                out.push_str(before);
                out.push_str(arg);
                rest = after;
            }
            None => break,
        }
    }
    out.push_str(rest);
    out
}

/// `{}` placeholders in `template`, filled in order from `args`.
pub fn fill_args(template: &str, args: &[&str]) -> String {
    fill_all(template, args)
}

/// Fills a phrase chosen at run time, which `format!` cannot take:
/// `fill!(phrase, a, b)`.
#[macro_export]
macro_rules! fill {
    ($template:expr $(, $arg:expr)* $(,)?) => {{
        let args: ::std::vec::Vec<::std::string::String> = ::std::vec![
            $(::std::string::ToString::to_string(&$arg)),*
        ];
        let refs: ::std::vec::Vec<&str> =
            args.iter().map(::std::string::String::as_str).collect();
        $crate::fill_args($template, &refs)
    }};
}

/// A phrase with one placeholder.
pub fn fill1(template: &str, a: impl std::fmt::Display) -> String {
    let a = a.to_string();
    fill_all(template, &[&a])
}

/// A phrase with two placeholders.
pub fn fill2(template: &str, a: impl std::fmt::Display, b: impl std::fmt::Display) -> String {
    let (a, b) = (a.to_string(), b.to_string());
    fill_all(template, &[&a, &b])
}

/// A phrase with three placeholders.
pub fn fill3(
    template: &str,
    a: impl std::fmt::Display,
    b: impl std::fmt::Display,
    c: impl std::fmt::Display,
) -> String {
    let (a, b, c) = (a.to_string(), b.to_string(), c.to_string());
    fill_all(template, &[&a, &b, &c])
}

/// A phrase with four placeholders.
pub fn fill4(
    template: &str,
    a: impl std::fmt::Display,
    b: impl std::fmt::Display,
    c: impl std::fmt::Display,
    d: impl std::fmt::Display,
) -> String {
    let (a, b, c, d) = (a.to_string(), b.to_string(), c.to_string(), d.to_string());
    fill_all(template, &[&a, &b, &c, &d])
}

/// Which language the system speaks.
#[cfg(windows)]
fn system_language() -> Lang {
    // Declared here rather than pulling in a crate for one call: kernel32 is
    // linked into every Windows program already.
    unsafe extern "system" {
        fn GetUserDefaultUILanguage() -> u16;
    }
    // SAFETY: no arguments, and the return value is a plain LANGID.
    let id = unsafe { GetUserDefaultUILanguage() };
    // The low ten bits are the primary language; 0x04 is Chinese.
    if id & 0x3ff == 0x04 {
        Lang::Zh
    } else {
        Lang::En
    }
}

#[cfg(not(windows))]
fn system_language() -> Lang {
    std::env::var("LANG")
        .ok()
        .and_then(|v| Lang::parse(&v))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_translation_is_a_compile_error() {
        // Every key answers in both languages, and no answer is empty.
        for lang in Lang::ALL {
            for key in [Key::EXTRACT, Key::CLI_ABOUT, Key::MSG_PROGRESS_SAVED] {
                assert!(!key.text(lang).is_empty());
            }
        }
    }

    #[test]
    fn tags_and_names_round_trip() {
        assert_eq!(Lang::parse("zh-CN"), Some(Lang::Zh));
        assert_eq!(Lang::parse("ZH_tw"), Some(Lang::Zh));
        assert_eq!(Lang::parse("en-US"), Some(Lang::En));
        assert_eq!(Lang::parse("fr"), None);
        assert_eq!(Lang::Zh.tag(), "zh");
    }

    #[test]
    fn entries_agree_with_their_number() {
        assert_eq!(entries(Lang::En, 1), "1 entry");
        assert_eq!(entries(Lang::En, 1_200), "1200 entries");
        assert_eq!(entries(Lang::Zh, 1), "1 个条目");
        assert_eq!(
            duration(Lang::En, Duration::from_secs(2)),
            "less than 5 seconds"
        );
        assert_eq!(duration(Lang::Zh, Duration::from_secs(2)), "不到 5 秒");
        assert_eq!(duration(Lang::Zh, Duration::from_secs(61)), "2 分钟");
        assert_eq!(
            duration(Lang::En, Duration::from_secs(3 * 3600 + 125)),
            "3 hours 2 minutes"
        );
    }
}
