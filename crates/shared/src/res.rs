//! Embedded resources.

pub static ICON_SMALL: &[u8] = include_bytes!("../../../res/icon/64x64.png");

pub static ICON_MEDIUM: &[u8] = include_bytes!("../../../res/icon/256x256.png");

#[cfg(feature = "release")]
pub fn licenses() -> Vec<zng::third_party::LicenseUsed> {
    zng_tp_licenses::include_bundle!()
}
