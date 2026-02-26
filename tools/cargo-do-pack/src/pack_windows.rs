use std::path::PathBuf;

/// Fragment of .iss file that declares tasks for enabling file associations
pub(crate) fn iss_tasks() {
    // Name: assoc_png; Description: .PNG; GroupDescription: {cm:FileAssociations}; Check: not WizardSilent
    for [name, exts, _] in formats() {
        let ext = exts.split(',').next().unwrap();
        println!(
            "Name: assoc_{ext}; Description: {name}; GroupDescription: {{cm:FileAssociations}}; Check: not WizardSilent"
        );
    }
}

/// Fragment of .iss file that declares Windows Registry entries to associate files
pub(crate) fn iss_registry() {
    // Root: HKCR; Subkey: ".png"; ValueType: string; ValueData: "${ZR_PKG_NAME}.Image"; Flags: uninsdeletevalue; Tasks: assoc_png
    let formats = formats();
    let pgk_name = std::env::var("ZR_PKG_NAME").expect("expected ZR_PKG_NAME");
    for [_, exts, _] in formats {
        let exts = exts.split(',').collect::<Vec<_>>();
        for ext in &exts {
            println!(
                r#"Root: HKCR; Subkey: ".{ext}"; ValueType: string; ValueData: "{pgk_name}.Image"; Flags: uninsdeletevalue; Tasks: assoc_{}"#,
                exts[0]
            );
        }
    }
}

/// [[name, extensions, mimes]]
fn formats() -> Vec<[String; 3]> {
    let mut path = PathBuf::from("target/release/image-viewer.exe");
    if !path.exists() {
        path = PathBuf::from("target/debug/image-viewer.exe");
        if !path.exists() {
            tools_util::die!("not built");
        }
    }
    let out = std::process::Command::new(path)
        .arg("--supported-formats")
        .output()
        .unwrap();
    let out = String::from_utf8(out.stdout).unwrap();

    let mut r = vec![];
    for line in out.lines().skip(1) {
        let mut s = line.split("\t");
        if let Some(name) = s.next()
            && let Some(exts) = s.next()
            && let Some(mimes) = s.next()
        {
            r.push([name.to_owned(), exts.to_owned(), mimes.to_owned()])
        } else {
            break;
        }
    }
    r
}
