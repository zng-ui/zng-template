//! Settings configuration.

use zng::config::*;

// others formats are available as Cargo features
type FileConfig = JsonConfig;
const CONFIG_FILE: &str = "config.json";
const SETTINGS_FILE: &str = "settings.json";

/// Initialize user config in the app context.
pub fn app_init() {
    // configs the user does not edit directly
    let default_config = ReadOnlyConfig::new(FileConfig::sync(zng::env::res(CONFIG_FILE)));
    let user_config = FileConfig::sync(zng::env::config(CONFIG_FILE));
    let config = FallbackConfig::new(user_config, default_config);

    // configs the user edits directly (all keys with "settings." prefix)
    let default_settings = ReadOnlyConfig::new(FileConfig::sync(zng::env::res(SETTINGS_FILE)));
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

    lang::init();
}

/// Lang config
pub mod lang {
    use zng::{l10n::Lang, prelude::*};

    pub const CONFIG_KEY: &str = "settings.lang";

    /// Config placeholder for [`L10N::sys_lang`].
    pub const SYSTEM_LANG: Lang = lang!("system");

    /// Bind the `L10n.app_lang` to the setting.
    pub(crate) fn init() {
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
