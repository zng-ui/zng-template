// Don't show console window in Windows release builds started from the Windows Explorer.
#![cfg_attr(feature = "release", windows_subsystem = "windows")]

mod cli;
mod config;
mod crash;
mod log;

use zng::prelude::*;

fn main() {
    // find app dirs
    zng::env::init("{{qualifier}}", "{{org}}", "{{app}}");
    // run and exit as CLI, or inits log and `shared::env`
    cli::run();
}
