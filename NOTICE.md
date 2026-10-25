# Notice

This repository is a modified fork of [xevrion/hatchzip](https://github.com/xevrion/hatchzip),
made from upstream commit `dd4f52f` (release v0.1.3, 2026-10-24).

Original project: HatchZip, by Yash Bavadiya, licensed MIT OR Apache-2.0.
The original license files (`LICENSE-MIT`, `LICENSE-APACHE`) are kept unchanged,
and the RAR decoder remains under its own license in `LICENSE-UNRAR`.

## What this fork changes

- Adds the `crates/i18n` crate: a catalog of every phrase the windows and the
  command line show, in English and Simplified Chinese, plus the language state.
- Adds `crates/gui/ui/i18n.slint`, a Slint global that Rust fills from the
  catalog at startup and again when the language changes.
- Localizes the desktop app, the command line help and messages, and the
  right-click menu labels.
- Adds a remembered language setting (`lang` in `settings.json`), a language
  menu in the desktop app, and the `HATCHZIP_LANG` environment variable.

No change was made to the extraction engine, the journal, or the code that
shrinks and removes archive bytes.

## Building with RAR support

The release builds include the `rar` cargo feature, which compiles UnRAR's C++
source. UnRAR is freeware for extracting RAR archives and may not be used to
build a RAR-compatible archiver; see `LICENSE-UNRAR`.
