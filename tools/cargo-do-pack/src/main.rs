use tools_util::*;

mod pack_android;
mod pack_deb;
mod pack_windows;

fn main() {
    let (arg_cmd, args) = args();
    match arg_cmd.as_str() {
        "deb" => deb(args),
        "android" => android(args),
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

/// do-pack windows [--iss-tasks] [--iss-registry]
fn windows(args: Vec<String>) {
    let (_, options, _) = split_args(&args, &[], &["--iss-tasks", "--iss-registry"], true, false);

    if options.contains_key("--iss-tasks") {
        return pack_windows::iss_tasks();
    }

    if options.contains_key("--iss-registry") {
        return pack_windows::iss_registry();
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
