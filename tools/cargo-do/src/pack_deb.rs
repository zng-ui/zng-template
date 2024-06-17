//! Helpers for `cargo do pack deb`

use crate::util::{self, *};
use std::{env, fs, io, path::Path};

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

pub(crate) fn changelog() {
    // https://manpages.debian.org/testing/dpkg-dev/deb-changelog.5.en.html

    let log = fs::read_to_string("CHANGELOG.md").unwrap_or_die("cannot read CHANGELOG.md");

    let mut lines = log.lines();
    let mut in_session = false;
    let mut session_blame = String::new();

    let package = env::var("ZR_PKG_NAME").unwrap();

    let mut line_n = 0;
    'outer: while let Some(line) = lines.next() {
        let mut line = line.trim();
        line_n += 1;

        // skip comments
        if line.starts_with("<!--") {
            while !line.ends_with("-->") {
                line = match lines.next() {
                    Some(l) => l.trim(),
                    None => break 'outer,
                }
            }
        }

        if let Some(version) = line.strip_prefix("## ") {
            let version = version.trim();

            let was_in_session = in_session;
            // 0.1.0 (2024-12-31)
            in_session = version.starts_with(|c: char| c.is_digit(10))
                && version.ends_with(')')
                && version.contains(" (");

            if was_in_session {
                println!("-- {session_blame}");
            }
            if in_session {
                //  git blame --porcelain -L n,+1 -- CHANGELOG.md
                let blame = util::cmd("git", &["blame", "--porcelain", "-L"])
                    .arg(format!("{line_n},+1"))
                    .args(&["--", "CHANGELOG.md"])
                    .output()
                    .success_or_die("cannot git blame CHANGELOG.md");

                let mut author = "";
                let mut author_mail = "";
                let mut author_time = "";
                // let mut author_tz = "";
                for line in blame.lines() {
                    if let Some(a) = line.strip_prefix("author ") {
                        author = a;
                    } else if let Some(a) = line.strip_prefix("author-mail ") {
                        author_mail = a;
                    } else if let Some(a) = line.strip_prefix("author-time ") {
                        author_time = a;
                    }
                    // else if let Some(a) = line.strip_prefix("author-tz ") {
                    //     author_tz = a;
                    // }
                }
                if author_mail == "<not.committed.yet>" {
                    die!("please commit changes to CHANGELOG.md first");
                }

                let date = util::cmd("date", &["-d"])
                    .arg(format!("@{author_time}"))
                    .output()
                    .success_or_die("cannot convert git blame date");

                session_blame = format!("{author} {author_mail} {date}");

                println!("\n{package} ({version}) unstable; urgency=low");
            }
        } else if in_session {
            println!("{line}");
        }
    }

    if in_session {
        println!("-- {session_blame}");
    }
}
