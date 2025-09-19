mod help;
#[macro_use]
mod util;
mod pack_android;
mod pack_deb;

use std::path::Path;

use util::*;

fn main() {
    let (arg_cmd, args) = args();
    match arg_cmd.as_str() {
        "check" => check(args),
        "fmt" => fmt(args),
        "l10n" => l10n(args),
        "pack" => pack(args),
        "build-release" | "build-r" => build_release(args),
        "build" => build(args),
        "build-ndk" => build_ndk(args),
        "run-release" | "run-r" => run_release(args),
        "run" => run(args),
        "update" => update(args),
        "help" | "--help" | "-h" | "" => help(args),
        _other => cmd("cargo", [arg_cmd].into_iter().chain(args))
            .status()
            .success_or_die("cannot run cargo"),
    }
}

/// do check [--release]
///    Cargo check, with release handling
fn check(args: Vec<String>) {
    let (_, options, args) = split_args(&args, &[], &["--release"], false, true);
    let mut cmd = if options.contains_key("--release") {
        cmd(
            "cargo",
            &[
                "check",
                "--release",
                "--no-default-features",
                "--features=release",
            ],
        )
    } else {
        cmd("cargo", &["check"])
    };
    cmd.args(args).status().success_or_die("cannot check");
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
    cmd(
        "cargo",
        &[
            "zng",
            "l10n",
            "--package",
            "t-app-t",
            "--output",
            "res/l10n",
            "--clean",
        ],
    )
    .args(args)
    .status()
    .success_or_die("cannot scrap l10n")
}

/// do pack <PACKAGE> [--no-build] [--dev]
///    Compile with release profile+features and package
///
///    ARGS
///       <PACKAGE>  - Name of a pack/{PACKAGE}
///       --no-build - Skips release build, you must call 'do build-release' before
///       --dev      - Pack the dev (debug) binary.
fn pack(args: Vec<String>) {
    // parse args
    let (package, options, args) =
        split_args(&args, &["PACKAGE"], &["--no-build", "--dev"], false, true);
    let package = package[0].as_str();

    if package == "deb" && options.contains_key("--changelog") {
        return pack_deb::changelog();
    }
    if package == "android" && options.contains_key("--locales") {
        return pack_android::locales();
    }

    if options.contains_key("--no-build") {
        println!("packing previous build");
    } else {
        println!("building release");
        if package == "android" {
            build_ndk(vec!["--release".to_owned()]);
        } else {
            let args = if options.contains_key("--dev") {
                vec!["--dev".to_owned()]
            } else {
                vec![]
            };
            build_release(args);
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
        .join(if options.contains_key("--dev") {
            "debug"
        } else {
            "release"
        })
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

/// do build-release, build-r [-z] [--bleed] [--dev]
///    Compile t-app-t release profile+features
///
///    ARGS
///       -z      - Optimize release for binary size
///       --bleed - Build with nightly compiler optimizations.
///       --dev   - Build with dev profile and release features.
fn build_release(args: Vec<String>) {
    let (_, options, args) = split_args(&args, &[], &["-z", "--bleed", "--dev"], true, true);
    let bleed = options.contains_key("--bleed");
    let dev = options.contains_key("--dev");
    let z = options.contains_key("-z");

    let mut cmd = std::process::Command::new("cargo");
    if bleed {
        cmd.arg("+nightly");
    }
    cmd.args([
        "build",
        if dev {
            "--profile=dev"
        } else {
            "--profile=release"
        },
        "--no-default-features",
        if z {
            "--features=release-z"
        } else {
            "--features=release"
        },
        "--package",
        "t-app-t",
    ])
    .args(args);

    if z {
        cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "z");
    }

    if bleed {
        // -Zshare-generics               - halves binary size
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
fn build(args: Vec<String>) {
    if args.iter().any(|a| a.starts_with("--release")) {
        die!("use `cargo do build-release` to build release")
    }
    cmd("cargo", ["build".to_owned()].into_iter().chain(args))
        .status()
        .success_or_die("cannot run cargo");
}

/// do run-release, run-r [--dev]
///    Compile and run the "portable" pack
///
///    ARGS
///       --dev   - Build with dev profile and release features.
fn run_release(mut args: Vec<String>) {
    let app_args = if let Some(i) = args.iter().position(|a| a == "--") {
        args.split_off(i)
    } else {
        vec![]
    };

    let dev = args.iter().any(|a| a == "--dev");

    println!("pack portable");
    let mut pack_args = vec!["portable".to_owned()];
    if dev {
        pack_args.push("--dev".to_owned());
    }
    pack(pack_args);

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
fn run(args: Vec<String>) {
    if args.iter().any(|a| a.starts_with("--release")) {
        die!("use `cargo do run-release` to run release")
    }
    cmd("cargo", ["run".to_owned()].into_iter().chain(args))
        .status()
        .success_or_die("cannot run cargo");
}

/// do build-ndk [--platform API-LEVEL] [--target TRIPLE] [--release] [-z] [--dev]
///    Compile t-app-t-mobile for Android using cargo-ndk
///
///    Default --platform is the latest installed
///    Default --target is all android targets installed
///    Default profile is 'dev'
///
///    ARGS
///       --release    - Build with release profile and features
///       --release -z - Build with release profile and features, and optimize for size
///       --dev        - Build with dev profile and release features
fn build_ndk(args: Vec<String>) {
    let (_, options, unknown_args) = split_args(
        &args,
        &[],
        &["--release", "-z", "--dev", "--platform", "--target"],
        false,
        true,
    );

    let mut args = vec![
        "ndk",
        "--manifest-path",
        "crates/t-app-t-mobile/Cargo.toml",
        "--output-dir",
        "target/build-ndk",
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

    let z = options.contains_key("-z");
    let feature = if z {
        "--features=release-z"
    } else {
        "--features=release"
    };

    args.extend_from_slice(&["build"]);
    if options.contains_key("--release") {
        args.extend_from_slice(&["--release", "--no-default-features", feature]);
    } else if options.contains_key("--dev") {
        args.extend_from_slice(&["--no-default-features", feature]);
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
    // optimize for size
    if z {
        cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "z");
    }
    if options.contains_key("--release") {
        // LTO "fat" have caused miscompilation for "aarch64-linux-android"
        // see https://github.com/zng-ui/zng/issues/488 for details.
        cmd.env("CARGO_PROFILE_RELEASE_LTO", "off");
    }
    let s = cmd.status().unwrap_or_die("cannot run cargo-ndk");
    if !s.success() {
        std::process::exit(s.code().unwrap_or(1));
    }
}

/// do update
///    Update dependencies and localization from dependencies
fn update(args: Vec<String>) {
    cmd("cargo", &["update"])
        .args(&args)
        .status()
        .success_or_die("cargo update failed");

    if args.is_empty() {
        // update l10n resources from external dependencies
        l10n(vec!["--no-local".to_owned(), "--no-pkg".to_owned()]);
    }
}

/// do help
///    Prints this help
fn help(_: Vec<String>) {
    self::help::print();
}
