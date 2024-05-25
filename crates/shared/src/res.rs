//! Embedded resources.

pub static ICON_SMALL: &[u8] = include_bytes!("../../../res/icon/small.png");

pub static ICON_MEDIUM: &[u8] = include_bytes!("../../../res/icon/medium.png");

#[cfg(feature = "release")]
pub fn licenses() -> Vec<zng::third_party::LicenseUsed> {
    zng_tp_licenses::include_bundle!()
}
