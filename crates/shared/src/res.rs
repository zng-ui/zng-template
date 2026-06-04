//! Embedded resources.

pub static ICON_SMALL: &[u8] = include_bytes!("../../../res/icon/64x64.png");

pub static ICON_MEDIUM: &[u8] = include_bytes!("../../../res/icon/256x256.png");

#[cfg(feature = "release")]
pub fn licenses() -> Vec<zng::third_party::LicenseUsed> {
    zng_tp_licenses::include_bundle!()
}

#[cfg(feature = "release")]
pub(crate) const L10N_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pack-l10n/l10n.tar"));

/// Extract embedded resources for live editing.
#[cfg(feature = "release")]
pub fn extract_l10n(dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = dir.with_added_extension(".tmp-dir");
    let tmp_tar = dir.with_added_extension(".tmp.tar");
    let _cleanup = zng::app::RunOnDrop::new(zng::clmv!(tmp_tar, tmp_dir, || {
        let _ = std::fs::remove_file(tmp_tar);
        let _ = std::fs::remove_dir_all(tmp_dir);
    }));

    std::fs::create_dir_all(&tmp_dir)?;
    std::fs::write(&tmp_tar, L10N_TAR)?;

    let s = std::process::Command::new("tar")
        .arg("-xf")
        .arg(tmp_tar)
        .arg("-C")
        .arg(&tmp_dir)
        .status()?;

    if !s.success() {
        return Err(format!("tar run failed, exit code: {:?}", s.code()).into());
    }

    std::fs::rename(tmp_dir.join("l10n"), dir)?;

    Ok(())
}
