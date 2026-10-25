//! The catalog: every phrase HatchZip shows, in English and Simplified
//! Chinese. One entry per phrase, so a translation that goes missing is a
//! compile error rather than an English line in a Chinese window.

macro_rules! catalog {
    ($( $key:ident => ($en:expr, $zh:expr) ),* $(,)?) => {
        /// One phrase HatchZip shows.
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        // The identifiers are the phrases' names, not types.
        #[allow(non_camel_case_types, dead_code)]
        pub enum Key {
            $($key),*
        }

        impl Key {
            /// The phrase in `lang`.
            pub fn text(self, lang: crate::Lang) -> &'static str {
                match (self, lang) {
                    $(
                        (Key::$key, crate::Lang::En) => $en,
                        (Key::$key, crate::Lang::Zh) => $zh,
                    )*
                }
            }
        }
    };
}

catalog! {
    // Numbers and time, shared by the windows and the command line.
    ENTRY_ONE => ("entry", "个条目"),
    ENTRY_MANY => ("entries", "个条目"),
    SECOND_ONE => ("second", "秒"),
    SECOND_MANY => ("seconds", "秒"),
    MINUTE_ONE => ("minute", "分钟"),
    MINUTE_MANY => ("minutes", "分钟"),
    HOUR_ONE => ("hour", "小时"),
    HOUR_MANY => ("hours", "小时"),
    LESS_THAN_FIVE_SECONDS => ("less than 5 seconds", "不到 5 秒"),
    WAITING_FOR_DISK => ("Waiting for disk", "正在等待磁盘"),
    REMAINING => ("remaining", "剩余"),
    RATE_LINE => ("{}/s, {} remaining", "{}/s,剩余 {}"),
    AMOUNTS_LINE => ("{} of {}", "{} / {}"),

    // Main window: menus and the empty screen.
    NAME_COLUMN => ("Name", "名称"),
    SIZE_COLUMN => ("Size", "大小"),
    MENU_FILE => ("File", "文件"),
    MENU_OPEN => ("Open…", "打开…"),
    MENU_QUIT => ("Quit", "退出"),
    MENU_SETTINGS => ("Settings", "设置"),
    MENU_HELP => ("Help", "帮助"),
    MENU_ABOUT => ("About HatchZip", "关于 HatchZip"),
    MENU_LANGUAGE => ("Language", "语言"),
    MENU_VERIFY => ("Verify archive before extracting", "解压前校验压缩包"),
    MENU_REMOVE => ("Remove the archive during extraction", "解压过程中删除压缩包"),
    LANG_ENGLISH => ("English", "English"),
    LANG_CHINESE => ("中文", "中文"),
    DROP_TO_OPEN => ("Drop the archive to open it", "松开以打开压缩包"),
    NO_ARCHIVE_OPEN => ("No archive open", "未打开压缩包"),
    EMPTY_DESCRIPTION => (
        "The archive is removed progressively during extraction, so only the space for the extracted files is required.",
        "解压时压缩包会被逐步删除,所以只需要为解压出的文件预留空间。"
    ),
    OPEN_ARCHIVE => ("Open archive…", "打开压缩包…"),
    OPEN_ANOTHER_ARCHIVE => ("Open another archive…", "打开其他压缩包…"),
    OPENING_PREFIX => ("Opening ", "正在打开 "),
    UNABLE_TO_OPEN_PREFIX => ("Unable to open ", "无法打开 "),
    FORMATS_RAR => (
        "Or drag an archive onto this window. Supported formats: ZIP, 7z, tar, tar.gz, RAR, and split archives.",
        "也可以把压缩包拖到此窗口。支持的格式:ZIP、7z、tar、tar.gz、RAR 和分卷压缩包。"
    ),
    FORMATS_NO_RAR => (
        "Or drag an archive onto this window. Supported formats: ZIP, 7z, tar, tar.gz, and split archives.",
        "也可以把压缩包拖到此窗口。支持的格式:ZIP、7z、tar、tar.gz 和分卷压缩包。"
    ),

    // Main window: an opened archive.
    ALL_EXTRACTED => ("All entries have been extracted.", "所有条目均已解压。"),
    ARCHIVE_EMPTY => ("This archive is empty.", "此压缩包为空。"),
    EXTRACT_TO => ("Extract to", "解压到"),
    BROWSE => ("Browse…", "浏览…"),
    RESERVED_SPACE => ("Reserved space", "预留空间"),
    GB => ("GB", "GB"),
    GB_INVALID => ("GB. Enter a number, such as 2 or 0.5.", "GB。请输入数字,例如 2 或 0.5。"),
    VERIFY_CHECKBOX => ("Verify archive before extracting", "解压前校验压缩包"),
    REMOVE_CHECKBOX => ("Remove the archive during extraction", "解压过程中删除压缩包"),
    NOTE_INPLACE => (
        "The archive is removed progressively during extraction. An interrupted extraction can be resumed.",
        "解压过程中压缩包会被逐步删除。中断的解压可以继续。"
    ),
    NOTE_KEEP => (
        "The archive is kept unchanged, so space is required for all extracted files. An interrupted extraction can be resumed.",
        "压缩包保持不变,因此需要为所有解压文件预留空间。中断的解压可以继续。"
    ),
    TEST => ("Test", "测试"),
    RESTORE_ARCHIVE => ("Restore archive", "还原压缩包"),
    EXTRACT => ("Extract", "解压"),
    RESUME_EXTRACTION => ("Resume extraction", "继续解压"),
    FINISH => ("Finish", "完成"),

    // The disk space panel.
    DISK_SPACE => ("Disk space", "磁盘空间"),
    CALCULATING => ("Calculating…", "正在计算…"),
    AVAILABLE => ("Available", "可用"),
    REQUIRED_REMOVED => ("Required, archive removed", "需要(删除压缩包)"),
    REQUIRED_KEPT => ("Required, archive kept", "需要(保留压缩包)"),
    RESERVED => ("Reserved", "预留"),
    AVAILABLE_AFTER => ("Available after extraction", "解压后可用"),
    NO_ENTRIES_REMAINING => ("No entries remaining", "没有剩余条目"),

    // Buttons shared by the windows.
    BACK => ("Back", "返回"),
    CANCEL => ("Cancel", "取消"),
    CLOSE => ("Close", "关闭"),
    PAUSING => ("Pausing…", "正在暂停…"),
    PAUSE => ("Pause", "暂停"),
    RESUME => ("Resume", "继续"),
    CHECK_AGAIN => ("Check again", "重新检查"),
    RETRY => ("Retry", "重试"),
    OPEN_FOLDER => ("Open folder", "打开文件夹"),

    // The password prompt.
    PASSWORD_REQUIRED => ("Password required", "需要密码"),
    PASSWORD_FOR_PREFIX => ("Enter the password for ", "请输入 "),
    PASSWORD_FOR_SUFFIX => (".", " 的密码。"),
    PASSWORD => ("Password", "密码"),
    INCORRECT_PASSWORD => ("Incorrect password.", "密码错误。"),
    OK => ("OK", "确定"),

    // About.
    ABOUT_TITLE => ("About HatchZip", "关于 HatchZip"),
    VERSION_PREFIX => ("Version ", "版本 "),
    ABOUT_DESCRIPTION => (
        "Extracts large archives in place. The archive is removed progressively during extraction, so only the space for the extracted files is required.",
        "就地解压大型压缩包。解压过程中压缩包会被逐步删除,所以只需要为解压出的文件预留空间。"
    ),
    LICENSE => ("Licensed under MIT or Apache-2.0", "以 MIT 或 Apache-2.0 许可证发布"),

    // The right-click menu, named after each system's file manager.
    SHELL_MENU_LABEL => (
        "Show in the file manager's context menu",
        "在文件管理器的右键菜单中显示"
    ),
    SHELL_MENU_LABEL_EXPLORER => ("Show in the Explorer context menu", "在资源管理器右键菜单中显示"),
    SHELL_MENU_LABEL_FINDER => ("Show in the Finder context menu", "在访达右键菜单中显示"),
    SHELL_EXTRACT_HERE => ("Extract here", "解压到当前文件夹"),
    SHELL_EXTRACT_HERE_KEEP => ("Extract here (keep archive)", "解压到当前文件夹(保留压缩包)"),
    SHELL_EXTRACT_TO => ("Extract to a new folder", "解压到新文件夹"),
    SHELL_TEST => ("Test archive", "测试压缩包"),
    SHELL_REGISTERED => ("Explorer: .{} files", "资源管理器:.{} 文件"),

    // A run: headings, details and outcomes.
    CHECKING_SPACE => ("Checking disk space…", "正在检查磁盘空间…"),
    READING_ARCHIVE => ("Reading archive… {}%", "正在读取压缩包… {}%"),
    HEADING_EXTRACTING => ("Extracting {}", "正在解压 {}"),
    HEADING_TESTING => ("Testing {}", "正在测试 {}"),
    HEADING_RESTORING => ("Restoring {}", "正在还原 {}"),
    HEADING_EXTRACTION_COMPLETE => ("Extraction complete", "解压完成"),
    HEADING_EXTRACTION_PAUSED => ("Extraction paused", "解压已暂停"),
    HEADING_EXTRACTION_FAILED => ("Extraction failed", "解压失败"),
    HEADING_INSUFFICIENT_SPACE => ("Insufficient disk space", "磁盘空间不足"),
    HEADING_TEST_COMPLETE => ("Test complete", "测试完成"),
    HEADING_TEST_FAILED => ("Test failed", "测试失败"),
    HEADING_RESTORE_COMPLETE => ("Restore complete", "还原完成"),
    HEADING_RESTORE_FAILED => ("Restore failed", "还原失败"),
    DETAIL_VERIFYING_TEST => ("Verifying every entry. No files are written.", "正在校验每个条目,不写入任何文件。"),
    DETAIL_RESTORING => (
        "Converting it back to a standard archive. Extracted files are kept.",
        "正在还原为标准压缩包。已解压的文件会保留。"
    ),
    DETAIL_DESTINATION_PREFIX => ("Destination: {}", "目标位置:{}"),
    MSG_PAUSING => ("Pausing after the current batch…", "当前批次结束后暂停…"),
    MSG_VERIFY_BEFORE => ("Verifying archive before extracting…", "解压前正在校验压缩包…"),
    MSG_PROGRESS_SAVED => (
        "Progress is saved. The extraction can be resumed now or later.",
        "进度已保存,可以现在或稍后继续解压。"
    ),
    MSG_EXTRACTED_KEPT => ("All entries were extracted. The archive was kept.", "所有条目均已解压,压缩包已保留。"),
    MSG_EXTRACTED_REMOVED => ("All entries were extracted. The archive was removed.", "所有条目均已解压,压缩包已删除。"),
    MSG_TEST_PASSED_ONE => ("Test passed: 1 entry verified.", "测试通过:已校验 1 个条目。"),
    MSG_TEST_PASSED => ("Test passed: {} entries verified.", "测试通过:已校验 {} 个条目。"),
    MSG_RESTORED => (
        "Archive restored. It contains {} not yet extracted. Extracted files remain in {}.",
        "压缩包已还原。其中还有 {}尚未解压。已解压的文件仍在 {}。"
    ),
    MSG_PROGRESS_DISCARDED => (
        "Extraction progress discarded. The archive was not modified. Extracted files remain in {}.",
        "已丢弃解压进度。压缩包未被修改。已解压的文件仍在 {}。"
    ),
    MSG_FAILED_RESUMABLE => ("{} Progress is saved; the extraction can be resumed.", "{} 进度已保存,可以继续解压。"),
    MSG_CANCELLED_EXTRACT => ("Extraction cancelled. The archive was not modified.", "已取消解压,压缩包未被修改。"),
    MSG_CANCELLED_TEST => ("Test cancelled. The archive was not modified.", "已取消测试,压缩包未被修改。"),
    MSG_PARTIAL_TEST => (
        "this archive is partially extracted and cannot be tested. Resume or restore it first",
        "此压缩包已部分解压,无法测试。请先继续解压或还原它"
    ),
    VERDICT_SUFFICIENT => ("Sufficient disk space", "磁盘空间充足"),
    VERDICT_SHORT => ("Insufficient disk space. {} more is required.", "磁盘空间不足,还需要 {}。"),
    NOTE_REMOVE_ONLY => (
        "Removing the archive during extraction requires only {}.",
        "解压过程中删除压缩包只需要 {}。"
    ),
    NOTE_KEEP_ENOUGH => ("There is also enough space to keep the archive.", "空间也够保留压缩包。"),
    NOTE_SOLID_BLOCK => (
        "Most of the required space is one solid block, which can only be released after it is fully extracted.",
        "所需空间大部分来自一个固实块,只有把它完整解压后才能释放。"
    ),
    NOTE_OTHER_DRIVE => (
        "The destination is on another drive, so removing the archive does not free space there.",
        "目标位置在另一个驱动器上,删除压缩包不会在那里释放空间。"
    ),
    NOTE_KEPT_WHOLE => (
        "This drive cannot release space inside a file, so the compressed archive remains whole until extraction completes.",
        "此驱动器无法释放文件内部的空间,所以压缩包在解压完成前会保持完整。"
    ),
    REFUSAL_KEEP => (
        "{}. Turn off \"Remove the archive during extraction\" to extract it and keep the archive",
        "{}。关闭“解压过程中删除压缩包”即可保留压缩包进行解压"
    ),
    STEP_ALL_DONE => ("All {} archives completed.", "全部 {} 个压缩包已完成。"),
    STEP_ARCHIVE => ("Archive {} of {}", "第 {} 个压缩包,共 {} 个"),
    STEP_LEFT_ONE => ("1 more archive has not started", "还有 1 个压缩包未开始"),
    STEP_LEFT => ("{} more archives have not started", "还有 {} 个压缩包未开始"),
    ARCHIVE_REMAINING => ("Archive remaining: {}", "压缩包剩余:{}"),
    ADDED_REQUIRED_AVAILABLE => ("Required: {}. Available: {}.", "需要 {}。可用 {}。"),
    SPACE_LINE => (
        "Available: {}. Required: {}, plus {} reserved.",
        "可用 {}。需要 {},另预留 {}。"
    ),
    NOTICE_MENU_ADDED => ("Context menu entries added.", "已添加右键菜单项。"),
    NOTICE_MENU_REMOVED => ("Context menu entries removed.", "已移除右键菜单项。"),
    DIALOG_OPEN_TITLE => ("Open archive", "打开压缩包"),
    FILTER_ARCHIVES => ("Archives", "压缩包"),
    FILTER_ALL_FILES => ("All files", "所有文件"),
    DIALOG_OUTPUT_TITLE => ("Select destination folder", "选择目标文件夹"),
    SUMMARY_ARCHIVE => ("{} archive", "{} 压缩包"),
    SUMMARY_UNCOMPRESSED => ("{} uncompressed", "{} 解压后"),
    MODE_RESUME => (
        "This archive is partially extracted to {}.",
        "此压缩包已部分解压到 {}。"
    ),
    MODE_RESUME_KEPT => (
        "This archive is partially extracted to {}. The archive is kept.",
        "此压缩包已部分解压到 {}。压缩包已保留。"
    ),
    MODE_ORPHAN => (
        "All entries have been extracted to {}. Select Finish to remove the remaining temporary files.",
        "所有条目已解压到 {}。点击完成以删除剩余的临时文件。"
    ),

    // The command line: about and help.
    CLI_ABOUT => (
        "Extract large archives in place, removing the archive progressively.",
        "就地解压大型压缩包,压缩包会随解压逐步删除。"
    ),
    CLI_LONG_ABOUT_BASE => (
        "Extract large archives in place, removing the archive progressively.\n\n\
         HatchZip extracts an archive from its end and truncates each completed part from the \
         archive file, so the released space is reused for the next files. Free space is required \
         for the extracted files only, not for the files and the archive. The space plan is \
         checked before the archive is modified, and an interrupted extraction resumes where it \
         stopped when the same command is run again.\n\n\
         With --keep-archive, HatchZip extracts like any other tool and leaves the archive \
         unchanged; free space is then required for all extracted files.\n\n\
         Supported: ZIP (including ZIP64 and self-extracting .exe), 7z, uncompressed tar, \
         gzip-compressed tar (.tar.gz, .tgz), split archives (.001, .002, ...) of any of those",
        "就地解压大型压缩包,压缩包会随解压逐步删除。\n\n\
         HatchZip 从压缩包末尾开始解压,每解完一部分就从压缩包文件中截断这一部分,释放出的空间\
         供后续文件使用。所需空闲空间只用于解压出的文件,不需要同时容纳文件和压缩包。在修改\
         压缩包之前会先检查空间计划;中断的解压再次运行同一条命令即可从断点继续。\n\n\
         使用 --keep-archive 时,HatchZip 与其他工具一样正常解压,不改动压缩包;此时需要为\
         所有解压文件预留空间。\n\n\
         支持:ZIP(包括 ZIP64 和自解压 .exe)、7z、未压缩 tar、gzip 压缩的 tar\
         (.tar.gz、.tgz),以及上述格式的分卷压缩包(.001、.002、...)"
    ),
    CLI_LONG_ABOUT_RAR_TAIL => (
        ", and RAR, single files and volume sets (.part1.rar, .part2.rar, ... or .rar, \
         .r00, ...). A solid RAR archive is extracted only with --keep-archive.",
        ",RAR 的单文件和分卷集(.part1.rar、.part2.rar、... 或 .rar、.r00、...)。\
         固实 RAR 压缩包只能用 --keep-archive 解压。"
    ),
    CLI_LONG_ABOUT_NO_RAR_TAIL => (
        ". This build does not include RAR support; the release builds from hatchzip.dev \
         and github.com/xevrion/hatchzip/releases do.",
        "。此构建不包含 RAR 支持;hatchzip.dev 和 github.com/xevrion/hatchzip/releases \
         发布的正式版本包含该支持。"
    ),
    CLI_AFTER_LONG_HELP => (
        "Examples:\n  \
         hatchzip check game.zip              Show the space plan without modifying anything\n  \
         hatchzip extract game.zip            Extract to ./game, removing game.zip during extraction\n  \
         hatchzip extract game.zip --keep-archive\n                                       Extract to ./game and keep game.zip unchanged\n  \
         hatchzip check game.zip --keep-archive\n                                       Show whether a standard extraction fits\n  \
         hatchzip x game.7z.001 -o D:\\Games   Extract a split archive from any of its parts\n  \
         hatchzip extract a.zip b.7z c.tar.gz Extract several archives, one after another\n  \
         hatchzip extract big.zip --verify-first --keep-free 10GB\n  \
         hatchzip test game.7z                Verify every entry without writing files\n  \
         hatchzip restore game.zip            Restore a partially extracted archive\n\n\
         To resume an interrupted extraction, run the same command again. Progress is stored in \
         two files next to the archive (<archive>.hatchzip and <archive>.hatchzip-tail); do not \
         delete them while an extraction is incomplete. To stop permanently instead, run \
         restore: the entries not yet extracted become a standard archive again. An extraction \
         with --keep-archive resumes the same way and keeps the archive when resumed.\n\n\
         Several archives are processed one after another. extract checks the space for each one \
         just before it starts and stops at the first one that does not complete, then lists what \
         was extracted and what was not. check, test and list process every archive.\n\n\
         Exit status:\n  \
         0    success\n  \
         1    failure (progress, if any, is saved)\n  \
         2    invalid command line\n  \
         3    insufficient disk space; the archive was not modified\n  \
         4    the archive is damaged; no files were removed\n  \
         5    the archive is password protected and no correct password was provided\n  \
         130  paused with Ctrl+C; run the same command to resume\n\
         With several archives, the status is that of the first archive that did not succeed.\n\n\
         Encrypted archives (ZIP with ZipCrypto or AES, 7z with AES): HatchZip prompts for the \
         password, or reads --password or the HATCHZIP_PASSWORD environment variable. The \
         password is verified on one file before the archive is modified. It is never stored, so \
         resuming an interrupted extraction prompts again.",
        "示例:\n  \
         hatchzip check game.zip              只显示空间计划,不做任何修改\n  \
         hatchzip extract game.zip            解压到 ./game,解压过程中删除 game.zip\n  \
         hatchzip extract game.zip --keep-archive\n                                       解压到 ./game 并保持 game.zip 不变\n  \
         hatchzip check game.zip --keep-archive\n                                       查看普通解压是否放得下\n  \
         hatchzip x game.7z.001 -o D:\\Games   从任意分卷打开分卷压缩包\n  \
         hatchzip extract a.zip b.7z c.tar.gz 依次解压多个压缩包\n  \
         hatchzip extract big.zip --verify-first --keep-free 10GB\n  \
         hatchzip test game.7z                校验每个条目,不写入文件\n  \
         hatchzip restore game.zip            把部分解压的压缩包还原为标准压缩包\n\n\
         要继续中断的解压,再次运行同一条命令。进度保存在压缩包旁的两个文件里\
         (<archive>.hatchzip 和 <archive>.hatchzip-tail),解压未完成时不要删除它们。\
         如果想彻底放弃,请运行 restore:尚未解压的条目会重新成为标准压缩包。使用 \
         --keep-archive 的解压以同样方式继续,继续时仍保留压缩包。\n\n\
         多个压缩包会依次处理。extract 在每个压缩包开始前重新检查空间,第一个未能完成的\
         会停止整个运行,随后列出哪些已解压、哪些没有。check、test 和 list 会处理每一个\
         压缩包。\n\n\
         退出状态:\n  \
         0    成功\n  \
         1    失败(如有进度会被保存)\n  \
         2    命令行无效\n  \
         3    磁盘空间不足;压缩包未被修改\n  \
         4    压缩包已损坏;没有删除任何文件\n  \
         5    压缩包有密码保护,且未提供正确密码\n  \
         130  用 Ctrl+C 暂停;运行同一条命令继续\n\
         有多个压缩包时,状态取第一个未成功的压缩包。\n\n\
         加密压缩包(ZIP 的 ZipCrypto 或 AES,7z 的 AES):HatchZip 会提示输入密码,或读取 \
         --password 或 HATCHZIP_PASSWORD 环境变量。密码会在修改压缩包之前先在一个文件上验证,\
         且从不保存,所以继续中断的解压时会再次询问。"
    ),

    // The command line: subcommands and arguments.
    CLI_EXTRACT_ABOUT => ("Extract an archive, removing it progressively", "解压压缩包,并逐步删除它"),
    CLI_EXTRACT_LONG => (
        "Extract an archive, removing it progressively.\n\n\
         Prints the space plan and asks for confirmation before starting. After an interruption \
         by Ctrl+C, a crash or a full disk, run the same command to resume.\n\n\
         With --keep-archive the archive is not modified and no confirmation is needed; free \
         space is required for all extracted files.\n\n\
         Several archives are extracted one after another. The space for each one is checked \
         just before it starts, and the first one that does not complete stops the run.",
        "解压压缩包,并逐步删除它。\n\n\
         开始前会打印空间计划并请求确认。被 Ctrl+C、崩溃或磁盘写满中断后,运行同一条命令即可\
         继续。\n\n\
         使用 --keep-archive 时压缩包不会被修改,也无需确认;此时需要为所有解压文件预留空间。\n\n\
         多个压缩包会依次解压。每个压缩包开始前都会检查空间,第一个未能完成的会停止整个运行。"
    ),
    CLI_CHECK_ABOUT => ("Show the space plan for an archive without modifying it", "显示压缩包的空间计划,不做任何修改"),
    CLI_LIST_ABOUT => ("List the entries of an archive", "列出压缩包中的条目"),
    CLI_TEST_ABOUT => ("Verify every entry of an archive without writing files", "校验压缩包中的每个条目,不写入文件"),
    CLI_RESTORE_ABOUT => ("Restore a partially extracted archive to a standard archive", "把部分解压的压缩包还原为标准压缩包"),
    CLI_RESTORE_LONG => (
        "Restore a partially extracted archive to a standard archive.\n\n\
         An interrupted extraction leaves the archive containing only the entries not yet \
         extracted, with its index saved alongside, so only HatchZip can open it. restore \
         appends a new index for those entries, so 7-Zip, WinRAR, unzip and tar can open it \
         again. No data is moved; only the space for the new index is required. Files already \
         extracted remain in the destination folder. If restore is interrupted, run it again, or \
         run extract to resume the extraction instead.\n\n\
         An extraction started with --keep-archive never modified the archive, so restore only \
         discards its progress.",
        "把部分解压的压缩包还原为标准压缩包。\n\n\
         中断的解压会让压缩包里只剩尚未解压的条目,索引则保存在旁边的文件里,因此只有 HatchZip \
         能打开它。restore 会为这些条目追加新的索引,7-Zip、WinRAR、unzip 和 tar 就能重新打开。\
         不会移动任何数据,只需要容纳新索引的空间。已解压的文件仍留在目标文件夹中。如果 restore \
         被中断,再运行一次即可,或者改用 extract 继续解压。\n\n\
         用 --keep-archive 开始的解压从未修改压缩包,所以 restore 只会丢弃其进度。"
    ),
    CLI_COMPLETIONS_ABOUT => ("Print a shell completion script", "输出 shell 补全脚本"),
    CLI_COMPLETIONS_LONG => (
        "Print a shell completion script.\n\n\
         bash:        hatchzip completions bash > ~/.local/share/bash-completion/completions/hatchzip\n\
         zsh:         hatchzip completions zsh > \"${fpath[1]}/_hatchzip\"\n\
         fish:        hatchzip completions fish > ~/.config/fish/completions/hatchzip.fish\n\
         PowerShell:  hatchzip completions powershell >> $PROFILE",
        "输出 shell 补全脚本。\n\n\
         bash:        hatchzip completions bash > ~/.local/share/bash-completion/completions/hatchzip\n\
         zsh:         hatchzip completions zsh > \"${fpath[1]}/_hatchzip\"\n\
         fish:        hatchzip completions fish > ~/.config/fish/completions/hatchzip.fish\n\
         PowerShell:  hatchzip completions powershell >> $PROFILE"
    ),
    CLI_MAN_ABOUT => ("Print the man page (roff)", "输出 man 手册页(roff)"),
    CLI_MAN_LONG => (
        "Print the man page in roff format.\n\n\
         hatchzip man > ~/.local/share/man/man1/hatchzip.1",
        "以 roff 格式输出 man 手册页。\n\n\
         hatchzip man > ~/.local/share/man/man1/hatchzip.1"
    ),
    CLI_QUIET => ("Print only errors and prompts", "只打印错误和提示"),
    CLI_HELP_FLAG => ("Print help", "显示帮助"),
    CLI_VERSION_FLAG => ("Print version", "显示版本"),
    CLI_NOT_FOUND => (
        "{} does not exist. No archive was extracted",
        "{} 不存在。没有解压任何压缩包"
    ),
    CLI_PAUSE_NOTE => (
        "Pausing after the current batch. Partly written files are redone on resume. Press \
         Ctrl+C again to exit immediately.",
        "当前批次结束后暂停。写了一半的文件会在继续时重做。再次按 Ctrl+C 立即退出。"
    ),
    CLI_OUTPUT => (
        "Destination folder [default: a folder named after the archive, next to it]. With \
         several archives, each one is extracted into this folder",
        "目标文件夹[默认:压缩包旁的同名文件夹]。有多个压缩包时,每个都解压到此文件夹"
    ),
    CLI_PASSWORD => (
        "Password for an encrypted archive. If omitted, HatchZip prompts for it with hidden \
         input, which is preferred: this value is visible to other programs",
        "加密压缩包的密码。省略时 HatchZip 会以隐藏输入方式询问,这种方式更安全:此值对其他程序可见"
    ),
    CLI_KEEP_ARCHIVE => (
        "Keep the archive: extract without removing anything, like a standard extraction. Free \
         space is required for all extracted files",
        "保留压缩包:像普通解压一样不删除任何内容。需要为所有解压文件预留空间"
    ),
    CLI_VERIFY_FIRST => ("Verify every entry before modifying the archive", "修改压缩包之前校验每个条目"),
    CLI_KEEP_FREE => ("Disk space reserved during extraction, so the disk never fills completely", "解压期间预留的磁盘空间,避免磁盘被完全占满"),
    CLI_YES => (
        "Skip the confirmation prompt (required outside a terminal, unless --keep-archive is given)",
        "跳过确认提示(非终端环境必须使用,除非给了 --keep-archive)"
    ),
    CLI_ARCHIVE_ARG => ("The archive, or any part of a split archive. Several can be given", "压缩包,或分卷压缩包的任意一卷。可以给出多个"),
    CLI_RESTORE_ARCHIVE_ARG => ("The partially extracted archive, or any part of a split archive", "部分解压的压缩包,或分卷压缩包的任意一卷"),
    CLI_SHELL_ARG => ("The shell to write completions for", "要生成补全的 shell"),
    CLI_INVALID_SIZE => ("'{}' is not a valid size, such as 500MB or 2GB", "'{}' 不是有效的大小,例如 500MB 或 2GB"),
    CLI_UNKNOWN_UNIT => ("unknown unit in '{}'. Use B, KB, MB, GB or TB", "'{}' 中的单位未知。请使用 B、KB、MB、GB 或 TB"),

    // The command line: messages.
    CLI_ERROR => ("error:", "错误:"),
    CLI_NO_ENTRIES_LEFT => ("no entries remain in this archive", "此压缩包中没有剩余条目"),
    CLI_PARTIAL_TEST => (
        "this archive is partially extracted and cannot be tested. Run extract to resume it, or restore to undo it",
        "此压缩包已部分解压,无法测试。运行 extract 继续,或运行 restore 撤销"
    ),
    CLI_DAMAGED => ("the archive is damaged; no files were extracted", "压缩包已损坏;没有解压出任何文件"),
    CLI_REFUSAL_KEEP => (
        "{}. Run the command again with --keep-archive to extract it and keep the archive, or \
         extract it with another program",
        "{}。请加上 --keep-archive 重新运行以保留压缩包解压,或改用其他程序解压"
    ),
    CLI_YES_REQUIRED => (
        "HatchZip removes the archive during extraction; pass --yes to confirm, or --keep-archive \
         to keep it",
        "HatchZip 会在解压过程中删除压缩包;请加 --yes 确认,或加 --keep-archive 保留它"
    ),
    CLI_CONFIRM => (
        "\nExtract to {} and remove {} during extraction? [y/N] ",
        "\n解压到 {} 并在解压过程中删除 {}?[y/N] "
    ),
    CLI_CANCELLED => ("extraction cancelled. The archive was not modified", "已取消解压,压缩包未被修改"),
    CLI_PASSWORD_PROMPT => ("Password: ", "密码:"),
    CLI_INCORRECT_PASSWORD => ("Incorrect password.", "密码错误。"),
    CLI_PASSWORD_NO_TTY => ("Pass --password or set HATCHZIP_PASSWORD when not running in a terminal.", "非终端环境下请传入 --password 或设置 HATCHZIP_PASSWORD。"),
    CLI_CHECKING_PASSWORD => (
        "Checking the password by decoding '{}' ({}). This archive has no password check value, \
         so this may take a while.",
        "正在通过解码 '{}' ({})验证密码。此压缩包没有密码校验值,所以可能需要一些时间。"
    ),
    CLI_READING_BAR => ("Reading", "正在读取"),
    CLI_EXTRACTING_BAR => ("Extracting", "正在解压"),
    CLI_VERIFYING_BAR => ("Verifying", "正在校验"),
    CLI_ARCHIVE_PROGRESS => ("archive: {}", "压缩包:{}"),
    CLI_INSTALL_HANDLER => ("installing Ctrl+C handler", "安装 Ctrl+C 处理器"),
    CLI_RESUMING => ("  resuming: {} of {} remaining", "  继续:剩余 {} / {}"),
    CLI_REPORT_SIZE => ("{} uncompressed in {}", "解压后 {},共 {}"),
    CLI_AVAILABLE_LINE => ("  available                    {}", "  可用                         {}"),
    CLI_REQUIRED_LINE => ("  required                     {} (+{} reserved)", "  需要                         {}(另预留 {})"),
    CLI_STANDARD_NEEDS => ("  standard extraction needs    {}", "  普通解压需要                 {}"),
    CLI_AVAILABLE_AFTER => ("  available after              {}", "  解压后可用                   {}"),
    CLI_NOTE_KEPT => (
        "  note: the archive is kept, so space is required for all extracted files",
        "  注意:压缩包会保留,因此需要为所有解压文件预留空间"
    ),
    CLI_NOTE_OTHER_DRIVE => (
        "  note: the destination is on another drive, so removing the archive does not free space there",
        "  注意:目标位置在另一个驱动器上,删除压缩包不会在那里释放空间"
    ),
    CLI_NOTE_KEPT_WHOLE => (
        "  note: this drive cannot release space inside a file, so the compressed archive remains \
         whole until extraction completes and room is required for the archive and every \
         extracted file",
        "  注意:此驱动器无法释放文件内部的空间,所以压缩包在解压完成前保持完整,需要同时容纳\
         压缩包和所有解压文件"
    ),
    CLI_NOTE_KEEP_ENOUGH => (
        "  note: there is also enough space to keep the archive (--keep-archive)",
        "  注意:空间也够保留压缩包(--keep-archive)"
    ),
    CLI_INSUFFICIENT => (
        "Insufficient disk space: {} more is required. Free up space or lower --keep-free.",
        "磁盘空间不足:还需要 {}。请释放空间或降低 --keep-free。"
    ),
    CLI_WITHOUT_KEEP => (
        "Without --keep-archive, the archive is removed during extraction and only {} is required.",
        "不加 --keep-archive 时,压缩包会在解压过程中删除,只需要 {}。"
    ),
    CLI_SOLID_NOTE => (
        "Most of the required space is one solid block, which can only be released after it is \
         fully extracted. Archives created with smaller solid blocks require much less space.",
        "所需空间大部分来自一个固实块,只有把它完整解压后才能释放。用更小的固实块创建的压缩包\
         所需空间会少得多。"
    ),
    CLI_ORPHAN_CHECK => ("All entries are already extracted. Run extract to complete the cleanup.", "所有条目都已解压。运行 extract 完成清理。"),
    CLI_LIST_HEADER => ("Size  Name", "大小  名称"),
    CLI_LIST_SYMLINK => ("(symlink)", "(符号链接)"),
    CLI_LIST_LINK => ("(link to {})", "(链接到 {})"),
    CLI_LIST_FOOTER => (
        "{}, {} uncompressed, {} {} archive",
        "{},解压后 {},{} 的 {} 压缩包"
    ),
    CLI_TEST_PASSED => ("Test passed: {} verified.", "测试通过:已校验 {} 个。"),
    CLI_EXTRACT_DONE => ("Extraction complete: {} in {}. {}", "解压完成:{}已解压到 {}。{}"),
    CLI_EXTRACT_KEPT => ("The archive was kept.", "压缩包已保留。"),
    CLI_EXTRACT_REMOVED => ("The archive was removed.", "压缩包已删除。"),
    CLI_PAUSED => ("Extraction paused. Run the same command to resume.", "解压已暂停。运行同一条命令继续。"),
    CLI_PROGRESS_SAVED => ("Progress is saved. Run the same command to resume.", "进度已保存。运行同一条命令继续。"),
    CLI_RESTORE_KEPT => ("Extraction progress discarded. {} was kept and was not modified.", "已丢弃解压进度。{} 已保留且未被修改。"),
    CLI_RESTORE_DONE => (
        "Archive restored. {} is a standard {} archive again and contains the {} not yet extracted.",
        "压缩包已还原。{} 重新成为标准的 {} 压缩包,其中包含尚未解压的 {}。"
    ),
    CLI_FILES_REMAIN => ("Extracted files remain in {}.", "已解压的文件仍在 {}。"),
    CLI_ARCHIVE_N_OF => ("Archive {} of {}:", "第 {} 个压缩包,共 {} 个:"),
    CLI_ALL_EXTRACTED => ("All {} archives extracted.", "全部 {} 个压缩包已完成。"),
    CLI_ALL_TESTED => ("All {} archives passed.", "全部 {} 个压缩包均通过。"),
    CLI_CHECK_MULTI_NOTE => (
        "Each archive is checked against the space available now. Extracting one changes the \
         space available for the next; extract checks each one again just before it starts.",
        "每个压缩包都按当前可用空间检查。解压一个会改变下一个的可用空间;extract 会在每个\
         压缩包开始前重新检查。"
    ),
    CLI_STOPPED => ("Stopped at archive {} of {}.", "在第 {} 个压缩包处停止,共 {} 个。"),
    CLI_STATE_EXTRACTED => ("extracted", "已解压"),
    CLI_STATE_PAUSED => ("paused", "已暂停"),
    CLI_STATE_SAVED => ("{}, progress saved", "{},进度已保存"),
    CLI_STATE_PAUSED_SAVED => ("paused, progress saved", "已暂停,进度已保存"),
    CLI_STATE_NOT_EXTRACTED => ("not extracted", "未解压"),
    CLI_STATE_NOT_STARTED => ("not started", "未开始"),
    CLI_CONTINUE => (
        "To continue, run extract again with the archives that are not extracted. An archive \
         with saved progress resumes where it stopped.",
        "要继续,请对尚未解压的压缩包再次运行 extract。有已保存进度的压缩包会从断点继续。"
    ),
    CLI_RESUME_KEEP_NOTE => ("This extraction was started with --keep-archive; the archive is kept.", "此解压是用 --keep-archive 开始的;压缩包会保留。"),
    CLI_ORPHAN_DONE => ("Extraction complete. Files are in {}.", "解压完成。文件在 {} 中。"),
    CLI_ENTRIES_ONE => ("1 entry", "1 个条目"),
    CLI_ENTRIES_MANY => ("{} entries", "{} 个条目"),
}
