mod help;
#[macro_use]
mod util;
mod pack_deb;

use std::{fs, path::Path};

use util::*;

fn main() {
    let (arg_cmd, args) = args();
    match arg_cmd.as_str() {
        "l10n" => l10n(args),
        "pack" => pack(args),
        "build-r" => build_r(args),
        "run-r" => run_r(args),
        "help" | "--help" | "-h" | "" => help(args),
        _other => cmd("cargo", [arg_cmd].into_iter().chain(args))
            .status()
            .success_or_die("cannot run cargo"),
    }
}

/// do l10n
///    Scraps localization text
fn l10n(args: Vec<String>) {
    let _ = fs::remove_dir_all("res/l10n/template");
    cmd(
        "cargo",
        &["zng", "l10n", "--package", "gui", "--output", "res/l10n"],
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
        build_r(vec![]);
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
///    Compile with release profile+features
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
///    Compile and run the "cargo-do-run-r" pack
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

/// do help
///    Prints this help
fn help(_: Vec<String>) {
    self::help::print();
}
