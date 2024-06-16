//! Helpers for `cargo do pack deb`

use crate::util::*;
use std::{fs, io, path::Path};

pub fn depends() -> String {
    // fake staging to call pkg-shlibdeps
    let temp: &Path = "target/tmp/do/pack/deb_deps".as_ref();
    let _ = fs::remove_dir_all(&temp);
    (|| {
        fs::create_dir_all(temp)?;
        let debian = temp.join("debian");
        fs::create_dir(&debian)?;
        fs::write(debian.join("control"), "")?;
        Ok::<_, io::Error>(())
    })()
    .ok_or_die("cannot create temp dir");

    // call pkg-shlibdeps
    let exe = Path::new("target/release/t-app-t").canonicalize().unwrap();
    let exe = exe.to_string_lossy();
    let stdout = cmd("dpkg-shlibdeps", &["-O", &exe])
        .current_dir(&temp)
        .output()
        .success_or_die("dpkg-shlibdeps run failed");
    let _ = fs::remove_dir_all(&temp);

    // parse package dependencies
    let last_line = stdout.lines().rev().next().unwrap_or_default();
    match last_line.strip_prefix("shlibs:Depends=") {
        Some(deps) => deps.to_owned(),
        None => die!("dpkg-shlibdeps did not return dependencies"),
    }
}
