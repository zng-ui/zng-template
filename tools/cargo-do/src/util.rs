use std::{
    collections::HashMap,
    io::{self, Write},
    process::{Command, ExitStatus},
};

#[macro_export]
macro_rules! die {
    ($($println_args:tt)*) => {
        {
            $crate::util::print_error();
            eprintln!($($println_args)*);
            std::process::exit(102);
        }
    };
}

pub fn cmd<S, A>(program: &str, args: A) -> Command
where
    S: AsRef<str>,
    A: IntoIterator<Item = S>,
{
    let mut cmd = Command::new(program);
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    cmd
}

pub trait ResultExt<T, E> {
    fn unwrap_or_die(self, comment: &str) -> T;
    fn ok_or_die(self, comment: &str);
}
impl<T, E: std::error::Error> ResultExt<T, E> for Result<T, E> {
    fn unwrap_or_die(self, comment: &str) -> T {
        self.unwrap_or_else(|e| die!("{comment}\n       {e}"))
    }

    fn ok_or_die(self, comment: &str) {
        let _ = self.unwrap_or_die(comment);
    }
}

#[allow(unused)]
pub trait CmdOutputExt {
    /// Returns the stdout
    fn success_or_die(self, comment: &str) -> String;
}
impl CmdOutputExt for io::Result<std::process::Output> {
    fn success_or_die(self, comment: &str) -> String {
        match self {
            Ok(s) => {
                if !s.status.success() {
                    std::io::stderr().write_all(&s.stderr).unwrap();
                }
                handle_exit_status(comment, &s.status);
                String::from_utf8_lossy(&s.stdout).into_owned()
            }
            Err(e) => die!("{comment}\n       {e}"),
        }
    }
}

pub trait CmdStatusExt {
    fn success_or_die(self, comment: &str);
}
impl CmdStatusExt for io::Result<ExitStatus> {
    fn success_or_die(self, comment: &str) {
        match self {
            Ok(s) => handle_exit_status(comment, &s),
            Err(e) => die!("{comment}\n       {e}"),
        }
    }
}

fn handle_exit_status(comment: &str, s: &ExitStatus) {
    if !s.success() {
        die!(
            "{comment}\n       command exited with error code {}",
            s.code().unwrap_or(0)
        )
    }
}

/// Prints "error: "
pub fn print_error() {
    const BOLD_RED: &str = "\x1B[1;31m";
    const BOLD_WHITE: &str = "\x1B[1;37m";
    const CLEAR: &str = "\x1B[0m";
    eprint!("{BOLD_RED}error{BOLD_WHITE}: {CLEAR}");
}

pub fn args() -> (String, Vec<String>) {
    if !std::env::current_dir()
        .unwrap()
        .join("tools/cargo-do/Cargo.toml")
        .exists()
    {
        die!("Please call cargo do only in the project root");
    }

    let mut args = std::env::args().skip(1);
    let arg1 = args.next().unwrap_or_default();
    let args: Vec<_> = args.collect();
    (arg1, args)
}

/// Returns (positional_args, options, unknowns)
pub fn split_args<'a, S: AsRef<str>>(
    args: &'a [S],
    arg_names: &'_ [&str],
    option_names: &'_ [&str],
    allow_no_positional: bool,
    allow_unknown_options: bool,
) -> (&'a [S], HashMap<&'a str, &'a str>, Vec<&'a str>) {
    if args.is_empty() {
        return (args, HashMap::new(), vec![]);
    }

    let (positional, options) = args.split_at(
        args.iter()
            .position(|a| !a.as_ref().starts_with('-'))
            .unwrap_or(args.len() - 1)
            + 1,
    );
    if positional.len() != arg_names.len() && !(positional.is_empty() && allow_no_positional) {
        die!("requested args: {}", arg_names.join(", "));
    }

    let mut options_map = HashMap::new();
    let mut unknowns = vec![];

    let mut last = "";
    for opt in options {
        let opt = opt.as_ref();
        if opt.starts_with('-') {
            let (key, val) = opt.split_once('=').unwrap_or((opt, ""));

            if !option_names.contains(&key) {
                if allow_unknown_options {
                    unknowns.push(opt);
                } else {
                    die!("unknown option '{key}'");
                }
            }

            options_map.insert(key, val);
            if val.is_empty() {
                last = key;
            }
        }
        if last.is_empty() {
            die!("unexpected value '{opt}'");
        } else {
            *options_map.get_mut(last).unwrap() = opt;
        }
    }

    (positional, options_map, unknowns)
}
