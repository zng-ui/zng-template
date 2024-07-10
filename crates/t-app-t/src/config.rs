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
    shared::env::init_config_reset(config.clone_boxed(), settings.clone_boxed());

    gui::settings::init();

    // split settings
    CONFIG.load(
        SwitchConfig::new()
            .with_prefix("settings.", settings)
            .with_prefix("", config),
    );
}
