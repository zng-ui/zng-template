mod help;
#[macro_use]
mod util;
mod pack_deb;

use util::*;

fn main() {
    let (arg_cmd, args) = args();
    match arg_cmd.as_str() {
        "l10n" => l10n(args),
        "pack" => pack(args),
        "build-r" => build_r(args),
        "run-r" => run_r(args),
        "help" | "" => help(args),
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

/// do pack [PACKAGE]
///    Compile with release profile+features and package
///
///    ARGS
///       [PACKAGE] - Name of a pack/{PACKAGE}
fn pack(args: Vec<String>) {
    // parse args
    let (package, _, args) = split_args(&args, &["PACKAGE"], &[], false, true);

    // build release
    build_r(vec![]);

    // pack
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
///    Compile with release profile+features and run
///    Also builds resources
fn run_r(mut args: Vec<String>) {
    let app_args = if let Some(i) = args.iter().position(|a| a == "--") {
        args.split_off(i)
    } else {
        vec![]
    };

    println!("build release");
    build_r(args);

    // must match zng::env::res default
    println!("build resources");
    #[allow(unused)]
    let res_target = "target/res";
    #[cfg(target_os = "linux")]
    let res_target = "target/etc";
    #[cfg(target_os = "macos")]
    let res_target = "target/Resources";
    cmd("cargo", &["zng", "res", "./res", res_target])
        .status()
        .success_or_die("resources build failed");

    #[cfg(not(windows))]
    let app = "target/release/t-app-t";
    #[cfg(windows)]
    let app = "target/release/t-app-t.exe";
    cmd(app, app_args)
        // .current_dir("target/release")
        .spawn()
        .ok_or_die("cannot spawn release app");
}

/// do help
///    Prints this help
fn help(_: Vec<String>) {
    self::help::print();
}
