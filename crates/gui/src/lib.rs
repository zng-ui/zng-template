// only enable hot-reload in "dev" desktop builds.
#[cfg(all(feature = "dev", not(any(target_os = "android", target_os = "ios")),))]
zng::hot_reload::zng_hot_entry!();

// crash handler only available in desktop builds.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod crash;

pub mod primary;
pub mod settings;
