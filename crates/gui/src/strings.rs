//! Filling each window's wording from the catalog. One list of setters, used
//! for every window, so a phrase added to one is added to all.

use hatchzip_i18n::{Key, Lang};
use slint::ComponentHandle;

use crate::{AboutWindow, MainWindow, Strings, TaskWindow};

macro_rules! fill {
    ($ui:expr, $lang:expr) => {{
        let lang: Lang = $lang;
        let s = $ui.global::<Strings>();

        s.set_menu_file(Key::MENU_FILE.text(lang).into());
        s.set_menu_open(Key::MENU_OPEN.text(lang).into());
        s.set_menu_quit(Key::MENU_QUIT.text(lang).into());
        s.set_menu_settings(Key::MENU_SETTINGS.text(lang).into());
        s.set_menu_help(Key::MENU_HELP.text(lang).into());
        s.set_menu_about(Key::MENU_ABOUT.text(lang).into());
        s.set_menu_language(Key::MENU_LANGUAGE.text(lang).into());
        s.set_lang_english(Key::LANG_ENGLISH.text(lang).into());
        s.set_lang_chinese(Key::LANG_CHINESE.text(lang).into());
        s.set_menu_verify(Key::MENU_VERIFY.text(lang).into());
        s.set_menu_remove(Key::MENU_REMOVE.text(lang).into());

        s.set_drop_to_open(Key::DROP_TO_OPEN.text(lang).into());
        s.set_no_archive_open(Key::NO_ARCHIVE_OPEN.text(lang).into());
        s.set_empty_description(Key::EMPTY_DESCRIPTION.text(lang).into());
        s.set_open_archive(Key::OPEN_ARCHIVE.text(lang).into());
        s.set_open_another_archive(Key::OPEN_ANOTHER_ARCHIVE.text(lang).into());
        s.set_opening_prefix(Key::OPENING_PREFIX.text(lang).into());
        s.set_unable_to_open_prefix(Key::UNABLE_TO_OPEN_PREFIX.text(lang).into());

        s.set_name_column(Key::NAME_COLUMN.text(lang).into());
        s.set_size_column(Key::SIZE_COLUMN.text(lang).into());
        s.set_all_extracted(Key::ALL_EXTRACTED.text(lang).into());
        s.set_archive_empty(Key::ARCHIVE_EMPTY.text(lang).into());
        s.set_extract_to(Key::EXTRACT_TO.text(lang).into());
        s.set_browse(Key::BROWSE.text(lang).into());
        s.set_reserved_space(Key::RESERVED_SPACE.text(lang).into());
        s.set_gb(Key::GB.text(lang).into());
        s.set_gb_invalid(Key::GB_INVALID.text(lang).into());
        s.set_verify_checkbox(Key::VERIFY_CHECKBOX.text(lang).into());
        s.set_remove_checkbox(Key::REMOVE_CHECKBOX.text(lang).into());
        s.set_note_inplace(Key::NOTE_INPLACE.text(lang).into());
        s.set_note_keep(Key::NOTE_KEEP.text(lang).into());
        s.set_test(Key::TEST.text(lang).into());
        s.set_restore_archive(Key::RESTORE_ARCHIVE.text(lang).into());
        s.set_extract(Key::EXTRACT.text(lang).into());
        s.set_resume_extraction(Key::RESUME_EXTRACTION.text(lang).into());
        s.set_finish(Key::FINISH.text(lang).into());

        s.set_disk_space(Key::DISK_SPACE.text(lang).into());
        s.set_calculating(Key::CALCULATING.text(lang).into());
        s.set_available(Key::AVAILABLE.text(lang).into());
        s.set_required_removed(Key::REQUIRED_REMOVED.text(lang).into());
        s.set_required_kept(Key::REQUIRED_KEPT.text(lang).into());
        s.set_reserved(Key::RESERVED.text(lang).into());
        s.set_available_after(Key::AVAILABLE_AFTER.text(lang).into());

        s.set_back(Key::BACK.text(lang).into());
        s.set_cancel(Key::CANCEL.text(lang).into());
        s.set_close(Key::CLOSE.text(lang).into());
        s.set_pausing(Key::PAUSING.text(lang).into());
        s.set_pause(Key::PAUSE.text(lang).into());
        s.set_resume(Key::RESUME.text(lang).into());
        s.set_check_again(Key::CHECK_AGAIN.text(lang).into());
        s.set_retry(Key::RETRY.text(lang).into());
        s.set_open_folder(Key::OPEN_FOLDER.text(lang).into());

        s.set_password_required(Key::PASSWORD_REQUIRED.text(lang).into());
        s.set_password_for_prefix(Key::PASSWORD_FOR_PREFIX.text(lang).into());
        s.set_password_for_suffix(Key::PASSWORD_FOR_SUFFIX.text(lang).into());
        s.set_password(Key::PASSWORD.text(lang).into());
        s.set_incorrect_password(Key::INCORRECT_PASSWORD.text(lang).into());
        s.set_ok(Key::OK.text(lang).into());

        s.set_about_title(Key::ABOUT_TITLE.text(lang).into());
        s.set_version_prefix(Key::VERSION_PREFIX.text(lang).into());
        s.set_about_description(Key::ABOUT_DESCRIPTION.text(lang).into());
        s.set_license(Key::LICENSE.text(lang).into());
    }};
}

/// The main window and the parts it uses.
pub fn apply(ui: &MainWindow, lang: Lang) {
    fill!(ui, lang);
}

/// The small window a right-click opens.
pub fn apply_task(ui: &TaskWindow, lang: Lang) {
    fill!(ui, lang);
}

/// The About window.
pub fn apply_about(ui: &AboutWindow, lang: Lang) {
    fill!(ui, lang);
}
