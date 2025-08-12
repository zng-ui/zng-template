fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

// link "c++_shared"
fn android() {
    println!("cargo:rustc-link-lib=c++_shared");
}
