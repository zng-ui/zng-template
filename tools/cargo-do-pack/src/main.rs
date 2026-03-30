use tools_util::*;

mod pack_android;
mod pack_deb;
#[cfg(windows)]
mod pack_windows;

fn main() {
    let (arg_cmd, args) = args();
    match arg_cmd.as_str() {
        "deb" => deb(args),
        "android" => android(args),
        #[cfg(windows)]
        "windows" => windows(args),
        "help" | "--help" | "-h" | "" => help(args),
        u => die!("unknown command {u}"),
    }
}

/// do-pack deb [--changelog] [--depends]
fn deb(args: Vec<String>) {
    let (_, options, _) = split_args(&args, &[], &["--changelog", "--depends"], true, false);

    if options.contains_key("--changelog") {
        return pack_deb::changelog();
    }
    if options.contains_key("--depends") {
        return pack_deb::depends();
    }
}

/// do-pack android [--locales]
fn android(args: Vec<String>) {
    let (_, options, _) = split_args(&args, &[], &["--locales"], true, false);

    if options.contains_key("--locales") {
        return pack_android::locales();
    }
}

/// do-pack windows [--iscc args] [--iss-languages]
#[cfg(windows)]
fn windows(args: Vec<String>) {
    let (_, options, args) = split_args(&args, &[], &["--iscc", "--iss-languages"], true, false);

    if options.contains_key("--iss-languages") {
        return pack_windows::iss_languages();
    }

    if options.contains_key("--iscc") {
        return pack_windows::iscc(args);
    }
}

/// do-pack help
///    Prints this help
fn help(_: Vec<String>) {
    print_help(
        "do-pack",
        "cargo do-pack <COMMAND> [args...]\n   Image Viewer package generator commands. implemented in 'tools/cargo-do-pack/src/main.rs'",
        include_str!("main.rs"),
    );
}
