//! Helpers for cargo do pack android

use std::{env, fs, path::PathBuf};

use crate::ResultExt;

/// Generates dummy values-{locale} resources to indicate support
pub(crate) fn locales() {
    let apk_res = PathBuf::from(env::var("ZR_TARGET_DD").unwrap());
    let mut create_default = false;
    for lang in fs::read_dir("res/l10n").unwrap_or_die("cannot read '../assets/res/l10n'") {
        let lang = lang.unwrap_or_die("cannot read 'res/l10n' entry").path();
        if lang.is_dir() {
            if let Some(lang) = lang.file_name().and_then(|f| f.to_str()) {
                if lang == "template" || lang.starts_with("pseudo") {
                    continue;
                }

                // en    -> values-en/
                // en-US -> values-en-rUS/
                let value = if let Some((lang, region)) = lang.split_once('-') {
                    if region.contains('-') {
                        die!("{lang} to locale folder name not implemented");
                    }
                    format!("values-{lang}-r{region}")
                } else {
                    format!("values-{lang}")
                };

                let apk_res_value = apk_res.join(value);
                if !apk_res_value.exists() {
                    write_dummy(apk_res_value);
                    create_default = true;
                }
            }
        }
    }

    if create_default {
        // a resource of the same name without lang (required)
        let apk_res_value = apk_res.join("values");
        if !apk_res_value.exists() {
            write_dummy(apk_res_value);
        }
    }
}
fn write_dummy(apk_res_value: PathBuf) {
    fs::create_dir(&apk_res_value).unwrap_or_die("cannot create value dir");
    let dummy = apk_res_value.join("strings.xml");
    fs::write(&dummy, DUMMY.as_bytes()).unwrap_or_die("cannot write dummy strings.xml");

    println!(
        "{}",
        dummy
            .strip_prefix(env::current_dir().unwrap())
            .unwrap()
            .display()
    );
}

const DUMMY: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="dummy">dummy</string>
</resources>"#;
