pub fn print() {
    const CODE: &str = include_str!("main.rs");

    println!("cargo do <COMMAND> [args...]\n");

    println!("   {{app}} project commands. implemented in 'tools/cargo-do/src/main.rs'\n");

    println!("COMMANDS\n");
    let mut print = false;
    for line in CODE.lines() {
        if let Some(c) = line.strip_prefix("///") {
            if c.starts_with(" do ") {
                print = true;
            }
            if print {
                println!("{c}");
            }
        } else {
            if print {
                println!();
            }
            print = false;
        }
    }

    println!(" do <other>\n    Redirects to cargo <other>");
}
