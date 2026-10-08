//! Embedded resources.

pub static ICON_SMALL: &[u8] = include_bytes!("../../../res/icon/64x64.png");

pub static ICON_MEDIUM: &[u8] = include_bytes!("../../../res/icon/256x256.png");

#[cfg(feature = "release")]
pub fn licenses() -> Vec<zng::third_party::LicenseUsed> {
    zng_tp_licenses::decode_embedding!()
}

#[cfg(feature = "release")]
pub(crate) const L10N_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pack-l10n/l10n.tar.zst"));

/// Extract embedded resources for live editing.
#[cfg(feature = "release")]
pub fn extract_l10n(dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = dir.with_added_extension(".tmp-dir");
    let _cleanup = zng::app::RunOnDrop::new(zng::clmv!(tmp_dir, || {
        let _ = std::fs::remove_dir_all(tmp_dir);
    }));
    std::fs::create_dir_all(&tmp_dir)?;

    zng::l10n::L10nTarData::Static(L10N_TAR).extract(&tmp_dir)?;

    std::fs::rename(tmp_dir.join("l10n"), dir)?;

    Ok(())
}