mod help;
#[macro_use]
mod util;
mod pack_deb;

use std::{fs, path::Path};

use util::*;

fn main() {
    let (arg_cmd, args) = args();
    match arg_cmd.as_str() {
        "fmt" => fmt(args),
        "l10n" => l10n(args),
        "pack" => pack(args),
        "build-r" => build_r(args),
        "build-ndk" => build_ndk(args),
        "run-r" => run_r(args),
        "help" | "--help" | "-h" | "" => help(args),
        _other => cmd("cargo", [arg_cmd].into_iter().chain(args))
            .status()
            .success_or_die("cannot run cargo"),
    }
}

/// do fmt
///    Calls cargo zng fmt
fn fmt(args: Vec<String>) {
    cmd("cargo", &["zng", "fmt"])
        .args(args)
        .status()
        .success_or_die("cannot zng fmt")
}

/// do l10n
///    Scraps localization text
fn l10n(args: Vec<String>) {
    let _ = fs::remove_dir_all("res/l10n/template");
    cmd(
        "cargo",
        &[
            "zng",
            "l10n",
            "--package",
            "t-app-t",
            "--output",
            "res/l10n",
        ],
    )
    .args(args)
    .status()
    .success_or_die("cannot scrap l10n")
}

/// do pack <PACKAGE> [--no-build]
///    Compile with release profile+features and package
///
///    ARGS
///       <PACKAGE>  - Name of a pack/{PACKAGE}
///       --no-build - Skips release build
fn pack(args: Vec<String>) {
    // parse args
    let (package, options, args) = split_args(&args, &["PACKAGE"], &["--no-build"], false, true);
    let package = package[0].as_str();

    if package == "deb" && options.contains_key("--changelog") {
        return pack_deb::changelog();
    }

    if options.contains_key("--no-build") {
        println!("skipping release build");
    } else {
        println!("building release");
        if package == "android" {
            build_ndk(vec!["--release".to_owned()]);
        } else {
            build_r(vec![]);
        }
    }

    // pack
    println!("packing");
    let mut pack_cmd = cmd(
        "cargo",
        &[
            "zng",
            "res",
            &format!("pack/{package}"),
            &format!("target/pack/{package}"),
            "--pack",
        ],
    );
    pack_cmd.args(&args);

    let name = format!("t-app-t{}", std::env::consts::EXE_SUFFIX);

    let app_path = Path::new("target")
        .canonicalize()
        .unwrap()
        .join("release")
        .join(&name)
        .display()
        .to_string();
    #[cfg(windows)]
    let app_path = app_path.trim_start_matches(r#"\\?\"#).replace('\\', "/");
    pack_cmd.env("DO_PACK_EXE", app_path);

    if package == "deb" {
        pack_cmd.env("DO_PACK_DEB_DEPENDS", pack_deb::depends());
    }

    pack_cmd
        .status()
        .success_or_die("cannot package, failed cargo zng res");
}

/// do build-r [--bleed]
///    Compile t-app-t release profile+features
///
///    ARGS
///       --bleed - Build with nightly compiler optimizations.
fn build_r(args: Vec<String>) {
    let (_, options, args) = split_args(&args, &[], &["--bleed"], true, true);
    let bleed = options.contains_key("--bleed");

    let mut cmd = std::process::Command::new("cargo");
    if bleed {
        cmd.arg("+nightly");
    }
    cmd.args([
        "build",
        "--release",
        "--no-default-features",
        "--features=release",
        "--package",
        "t-app-t",
    ])
    .args(args);

    if bleed {
        // -Zshare-generics - halves binary size
        // -C link-args=-znostart-stop-gc - Fixes build error
        cmd.env(
            "RUSTFLAGS",
            format!(
                "{} -Z share-generics -C link-args=-znostart-stop-gc",
                std::env::var("RUSTFLAGS").unwrap_or_default()
            ),
        );
    }
    cmd.status().success_or_die("release build failed");
}

/// do run-r
///    Compile and run the "portable" pack
fn run_r(mut args: Vec<String>) {
    let app_args = if let Some(i) = args.iter().position(|a| a == "--") {
        args.split_off(i)
    } else {
        vec![]
    };

    println!("pack portable");
    args.push("portable".to_owned());
    pack(args);

    let path = format!(
        "target/pack/portable/t-app-t{}",
        std::env::consts::EXE_SUFFIX
    );
    println!("\nrunning {path}");
    let s = cmd(&path, app_args)
        .status()
        .unwrap_or_die("cannot run app");
    if !s.success() {
        std::process::exit(s.code().unwrap_or(1));
    }
}

/// do build-ndk [--platform API-LEVEL] [--target TRIPLE] [--release]
///    Compile t-app-t-mobile for Android using cargo-ndk
///
///    Default --platform is the latest installed
///    Default --target is all android targets installed
fn build_ndk(args: Vec<String>) {
    let (_, options, unknown_args) = split_args(
        &args,
        &[],
        &["--release", "--platform", "--target"],
        false,
        true,
    );

    // avoid relative path, see issue https://github.com/bbqsrc/cargo-ndk/issues/139
    let output_dir = std::env::current_dir()
        .unwrap()
        .join("target/build-ndk")
        .display()
        .to_string();

    let mut args = vec![
        "ndk",
        "--manifest-path",
        "crates/t-app-t-mobile/Cargo.toml",
        "--output-dir",
        &output_dir,
    ];
    if let Some(p) = options.get("--platform") {
        args.extend_from_slice(&["--platform", p[0]]);
    }

    let installed_targets;
    if let Some(t) = options.get("--target") {
        for t in t {
            args.extend_from_slice(&["--target", t]);
        }
    } else {
        installed_targets = cmd("rustup", &["target", "list", "--installed"])
            .output()
            .success_or_die("cannot get installed targets");

        let mut any = false;
        for line in installed_targets.lines() {
            if line.contains("-android") {
                any = true;
                args.extend_from_slice(&["--target", line]);
            }
        }

        if !any {
            die!("no android target installed, rustup target add aarch64-linux-android")
        }
    }

    args.extend_from_slice(&["build"]);
    if options.contains_key("--release") {
        args.extend_from_slice(&["--release", "--no-default-features", "--features=release"]);
    }
    args.extend_from_slice(&unknown_args);

    let mut cmd = cmd("cargo", &args);
    // args required to build linkme
    cmd.env(
        "RUSTFLAGS",
        format!(
            "{} -Clink-arg=-z -Clink-arg=nostart-stop-gc",
            std::env::var("RUSTFLAGS").unwrap_or_default()
        ),
    );
    let s = cmd.status().unwrap_or_die("cannot run cargo-ndk");
    if !s.success() {
        std::process::exit(s.code().unwrap_or(1));
    }
}

/// do help
///    Prints this help
fn help(_: Vec<String>) {
    self::help::print();
}
