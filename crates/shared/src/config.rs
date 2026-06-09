//! App configuration from *env*, *config* and *settings*.
//!
//! - *env* - Defined from CLI, env variables and "env-save.env" files, aggregated in [`crate::env::args`].
//! - *config* - Defined in app resources and user config, not defined by the user directly (things like window state).
//! - *settings* - Configs the user can edit directly in the settings screen.

use zng::{config::*, prelude::*};

pub fn init() {
    init_config();
    init_l10n();
    init_render();
    #[cfg(feature = "release")]
    init_licenses();

    lang::bind();
}

/// Load config and settings.
fn init_config() {
    // others formats are available as Cargo features
    type FileConfig = JsonConfig;

    const CONFIG_FILE: &str = "config.json";
    const SETTINGS_FILE: &str = "settings.json";

    // configs the user does not edit directly
    let default_config = FileConfig::read(zng::env::res(CONFIG_FILE));
    let user_config = FileConfig::sync(zng::env::config(CONFIG_FILE));
    let config = FallbackConfig::new(user_config, default_config);

    // configs the user edits directly (all keys with "settings." prefix)
    let default_settings = FileConfig::read(zng::env::res(SETTINGS_FILE));
    let user_settings = FileConfig::sync(zng::env::config(SETTINGS_FILE));
    let settings = FallbackConfig::new(user_settings, default_settings);

    // init reset service
    crate::env::init_config_reset(config.clone_boxed(), settings.clone_boxed());

    // split settings
    CONFIG.load(
        SwitchConfig::new()
            .with_prefix("settings.", settings)
            .with_prefix("", config),
    );
}

/// Load localization.
fn init_l10n() {
    if let Some(dir) = &crate::env::args().lang_dir {
        L10N.load_dir(dir);
        return;
    }
    #[cfg(feature = "release")]
    L10N.load_tar(crate::res::L10N_TAR);
}

/// Configure render.
fn init_render() {
    // set render mode
    WINDOWS
        .default_render_mode()
        .set(crate::env::args().render_mode);

    // disable shader cache
    if crate::env::args().no_shader_cache {
        WINDOWS.default_cache_shaders().set(false);
    }

    // use ANGLE in all app windows
    #[cfg(windows)]
    if !crate::env::args().no_angle {
        zng_view_angle::register_root_extender();
    }
}

/// Load licenses.
#[cfg(feature = "release")]
fn init_licenses() {
    // register embedded licenses, used by the default `OPEN_LICENSES_CMD` screen.
    zng::third_party::LICENSES.register(crate::res::licenses);

    #[cfg(windows)]
    zng_view_angle::register_license();
}

/// Lang setting.
pub mod lang {
    use zng::{l10n::Lang, prelude::*};

    pub const CONFIG_KEY: &str = "settings.lang";

    /// Config placeholder for [`L10N::sys_lang`].
    pub const SYSTEM_LANG: Lang = lang!("system");

    /// Bind `L10n.app_lang` to the setting.
    pub(super) fn bind() {
        let actual_lang = expr_var! {
            let lang = #{CONFIG.get(CONFIG_KEY, SYSTEM_LANG)};
            if lang == &SYSTEM_LANG {
                #{L10N.sys_lang()}.clone()
            } else {
                lang.clone().into()
            }
        };
        let app_lang = L10N.app_lang();
        actual_lang.set_bind(&app_lang).perm();
        app_lang.hold(actual_lang).perm();
    }
}
