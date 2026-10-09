use std::env;

const HELP: &str = r#"
Example ''custom' tool, used by `cargo zng res` and `cargo do pack`.

Just add a file to `tools/cargo-zng-res/src/bin` to define a new tool.
See https://zng-ui.github.io/doc/cargo_zng/index.html#authoring-tools.
"#;
fn main() {
    if env::var("ZR_HELP").is_ok() {
        println!("{HELP}");
        return;
    }
    // ..
}
