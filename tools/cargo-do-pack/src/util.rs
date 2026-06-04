use std::fs;

use tools_util::*;

/// Get release langs
pub fn release_langs() -> Vec<String> {
    let mut r = vec![];
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

            if has_ftl {
                r.push(name.into_owned());
            }
        }
    }
    r
}
