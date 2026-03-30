use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use tools_util::*;

fn inno_setup() -> PathBuf {
    if let Ok(p) = std::env::var("ISCC_PATH") {
        let path = PathBuf::from(&p);
        if !path.is_file() {
            die!("ISCC_PATH='{p}' does not point to an existing ISCC.exe file")
        }
        path
    } else {
        if let Ok(pf) = std::env::var("ProgramFiles(x86)") {
            let iscc = Path::new(pf.as_str()).join("Inno Setup 6/ISCC.exe");
            if iscc.is_file() {
                return iscc;
            }
        }
        die!("cannot find ISCC.exe in default location, set ISCC_PATH to resolve");
    }
}

/// Run ISCC.exe.
pub(crate) fn iscc(args: Vec<&str>) {
    cmd(inno_setup(), args)
        .status()
        .success_or_die("failed ISCC run");
}

/// Fragment of .iss file that declares installer languages
pub(crate) fn iss_languages() {
    // This script matches the InnoSetup language resources with l10n, automatically selecting
    // languages that the app support
    let inno_langs = inno_setup_langs().unwrap_or_die("cannot search InnoSetup languages");

    // match res/l10n with Inno compiler languages
    let mut matches = HashSet::new();
    for lang in l10n_langs().unwrap_or_die("cannot read 'res/l10n'") {
        let mut best = ("", "");
        let mut best_score = 0;
        for (file, inno_lang) in &inno_langs {
            let score = lang
                .split('-')
                .zip(inno_lang.split('-'))
                .take_while(|(a, b)| a == b)
                .count();
            if score > best_score {
                best_score = score;
                best = (file.as_str(), *inno_lang);
            }
        }
        if !best.0.is_empty() {
            matches.insert(best);
        }
    }

    // generate [Languages] ISS code
    for (file, lang) in matches {
        let id = lang.replace('-', "_");
        // ; Name: pt_br; MessagesFile: compiler:Languages\BrazilianPortuguese.isl
        println!(r"; Name: {id}; MessagesFile: compiler:{file}");
    }
}

fn l10n_langs() -> io::Result<Vec<String>> {
    let mut r = vec![];
    for lang in fs::read_dir("res/l10n")? {
        let lang = lang?.path();
        if lang.is_dir()
            && let Some(lang) = lang.file_name().and_then(|f| f.to_str())
        {
            if lang == "template" || lang.starts_with("pseudo") {
                continue;
            }
            let lang = lang.strip_prefix("-machine").unwrap_or(lang);
            r.push(lang.to_owned());
        }
    }
    Ok(r)
}

/// Returns [("file.isl", "unic-lang-id")]
fn inno_setup_langs() -> io::Result<Vec<(String, &'static str)>> {
    // InnoSetup language resources do not use a standard name, we need to parse each to get the
    // LanguageID value that is a Windows LCID and convert it to Unicode Language Identifier

    let inno = inno_setup();
    let inno_dir = inno.parent().unwrap();

    let mut r = vec![];
    if let Some(l) = isl_lang(&inno_dir.join("Default.isl")) {
        r.push(("Default.isl".to_owned(), l))
    }
    for file in fs::read_dir(inno_dir.join("Languages"))? {
        let file = file?.path();
        if file.is_file()
            && let Some(ext) = file.extension()
            && ext == "isl"
            && let Some(l) = isl_lang(&file)
        {
            let res = file.file_name().unwrap().to_str().unwrap();
            r.push((format!(r"Languages\{res}"), l));
        }
    }
    Ok(r)
}

fn isl_lang(isl: &Path) -> Option<&'static str> {
    for line in io::BufReader::new(fs::File::open(isl).ok()?).lines() {
        let line = line.ok()?;
        if let Some(code) = line.strip_prefix("LanguageID=$") {
            if let Ok(code) = u32::from_str_radix(code.trim_end(), 16)
                && let Ok(l) = <&'static lcid::LanguageId>::try_from(code)
            {
                return Some(l.name);
            }
            break;
        }
    }
    None
}
