//! Helpers for cargo do-pack android

use std::{env, fs, path::PathBuf};

use crate::{ResultExt, pack_common::release_l10n};

/// Generates dummy values-{locale} resources to indicate support
pub(crate) fn locales() {
    let apk_res = PathBuf::from(env::var("ZR_TARGET_DD").unwrap());

    let mut create_default = false;
    for lang in release_l10n() {
        let lang = lang.file_name().unwrap().to_str().unwrap();
        // pt-machine -> values-b+pt
        // pt-PT -> values-b+pt+PT
        let lang = lang.strip_prefix("-machine").unwrap_or(lang);
        let lang = lang.replace('-', "+");
        let value = format!("values-b+{lang}");

        let apk_res_value = apk_res.join(value);
        if !apk_res_value.exists() {
            write_dummy(apk_res_value);
            create_default = true;
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
