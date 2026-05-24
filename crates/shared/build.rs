fn main() {
    // Collect license text for all dependencies
    #[cfg(feature = "release")]
    {
        pack_l10n();

        let licenses = zng_tp_licenses::collect_cargo_about("../../.cargo/about.toml");
        zng_tp_licenses::write_bundle(&licenses);
    }
}

#[cfg(feature = "release")]
/// Pack l10n dir for embedding using `l10N.load_tar`.
fn pack_l10n() {
    println!("cargo::rerun-if-changed=pack-l10n");
    println!("cargo::rerun-if-changed=../res/l10n");

    let res_dir =
        std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("pack-l10n");
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("pack-l10n");

    let cargo_zng_res = std::process::Command::new("cargo")
        .arg("zng")
        .arg("res")
        .arg(res_dir)
        .arg(out_dir)
        .status()
        .expect("failed to pack l10n resources");
    assert!(cargo_zng_res.success());
}
