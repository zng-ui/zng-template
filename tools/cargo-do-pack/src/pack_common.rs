use std::{fs, path::Path};

use tools_util::*;

/// Copy only res/l10n dirs that have localization files, not just deps
pub fn l10n() {
    let mut copy = vec![];

    for lang_dir in fs::read_dir("res/l10n").unwrap_or_die("cannot read res/l10n") {
        let lang_dir = lang_dir.unwrap_or_die("cannot read res/l10n entry").path();
        if lang_dir.is_dir() {
            // skip pseudo* and template
            let name = lang_dir.file_name().unwrap().to_string_lossy();
            if name.starts_with("pseudo") || name == "template" {
                continue;
            }

            // check if dir actually has app translations
            let mut has_ftl = false;
            for entry in fs::read_dir(&lang_dir).unwrap_or_die("cannot read res/l10n entry") {
                let entry = entry.unwrap_or_die("cannot read res/l10n entry").path();
                if entry.is_file()
                    && let Some(e) = entry.extension()
                    && e.eq_ignore_ascii_case("ftl")
                {
                    has_ftl = true;
                    break;
                }
            }

            // copy
            if has_ftl {
                copy.push(lang_dir);
            }
        }
    }

    // generate .zr-copy instruction files so we don't need to implement dir copying
    if !copy.is_empty() {
        let target = std::env::var("ZR_TARGET_DD").unwrap_or_die("expected `ZR_TARGET_DD` env var");
        let target = Path::new(&target).join("l10n");
        fs::create_dir_all(&target).unwrap_or_die("cannot create target l10n dir");

        for lang_dir in copy {
            let file_name = lang_dir.file_name().unwrap().to_str().unwrap();
            let target = target.join(format!("{file_name}.zr-copy"));
            fs::write(target, lang_dir.display().to_string().as_bytes())
                .unwrap_or_die("cannot write lang zr-copy");
        }
    }
}
