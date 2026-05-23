//! Render configuration

use zng::window::WINDOWS;

/// Apply configuration in the app context.
pub fn init() {
    // set render mode
    WINDOWS
        .default_render_mode()
        .set(crate::env::args().render_mode);

    // disable shader cache
    if crate::env::args().no_shader_cache {
        WINDOWS.default_cache_shaders().set(false);
    }

    // register ANGLE
    #[cfg(windows)]
    {
        zng_view_angle::register_license();
        if !crate::env::args().no_angle {
            zng_view_angle::register_root_extender();
        }
    }
}
