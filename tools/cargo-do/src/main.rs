mod help;
#[macro_use]
mod util;
mod pack_deb;

use std::path::Path;

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
    cmd("cargo", &["zng", "l10n", "crates/**/*.rs", "res/l10n"])
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

    if options.contains_key("--no-build") {
        println!("skipping release build");
    } else {
        println!("building release");
        build_r(vec![]);
    }

    // pack
    println!("packing");
    let package = package[0].as_str();
    let mut cmd = cmd(
        "cargo",
        &[
            "zng",
            "res",
            &format!("pack/{package}"),
            &format!("target/pack/{package}"),
            "--pack",
        ],
    );
    cmd.args(&args);

    let name = format!("t-app-t{}", std::env::consts::EXE_SUFFIX);
    cmd.env(
        "T_APP_T",
        Path::new("target")
            .canonicalize()
            .unwrap()
            .join("release")
            .join(&name),
    );

    if package == "deb" {
        cmd.env("DO_PACK_DEB_DEPENDS", pack_deb::depends());
    }

    cmd.status()
        .success_or_die("cannot package, failed cargo zng res");
}

/// do build-r
///    Compile with release profile+features
fn build_r(args: Vec<String>) {
    cmd(
        "cargo",
        &[
            "build",
            "--release",
            "--no-default-features",
            "--features=release",
        ],
    )
    .args(args)
    .status()
    .success_or_die("release build failed");
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
