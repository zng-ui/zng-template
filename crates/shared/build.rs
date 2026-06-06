fn main() {
    #[cfg(feature = "release")]
    release::build();
}

#[cfg(feature = "release")]
mod release {
    pub fn build() {
        build_l10n();
        build_licenses();
    }

    // Pack l10n dir for embedding using `l10N.load_tar`.
    fn build_l10n() {
        println!("cargo::rerun-if-changed=pack-l10n");
        println!("cargo::rerun-if-changed=../res/l10n");

        let res_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
            .join("pack-l10n");
        let out_dir =
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("pack-l10n");

        let cargo_zng_res = std::process::Command::new("cargo")
            .arg("zng")
            .arg("res")
            .arg(res_dir)
            .arg(out_dir)
            .status()
            .expect("failed to pack l10n resources");
        assert!(cargo_zng_res.success());
    }

    // Collect license text for all dependencies
    fn build_licenses() {
        let licenses = zng_tp_licenses::collect_cargo_about_for(
            "../../.cargo/about.toml",
            "../t-app-t/Cargo.toml",
            "release",
        );
        // add extra licenses here
        zng_tp_licenses::write_embeddings(&licenses);
    }
}
