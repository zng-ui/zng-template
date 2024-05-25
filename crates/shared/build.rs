fn main() {
    collect_bundle_licenses();
}

/// Collect license text for all dependencies when built with `feature="bundle_licenses"`
fn collect_bundle_licenses() {
    #[cfg(feature = "release")]
    {
        let licenses = zng_tp_licenses::collect_cargo_about("../../.cargo/about.toml");
        zng_tp_licenses::write_bundle(&licenses);
    }
}
