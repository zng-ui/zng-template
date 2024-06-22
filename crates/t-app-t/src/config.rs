use zng::config::*;

// others formats are available as Cargo features
type FileConfig = JsonConfig;
const FILE: &str = "config.json";

/// Initialize user config in the app context.
pub fn app_init() {
    // read-only default config
    let app = ReadOnlyConfig::new(FileConfig::sync(zng::env::res(FILE)));
    // read-write config
    let user = FileConfig::sync(zng::env::config(FILE));

    // final setup
    let config = FallbackConfig::new(user, app);

    // create a control ref to the config, settings UI can use this to reset configs.
    shared::env::init_config_reset(config.clone_boxed());

    CONFIG.load(config);

    // also see https://github.com/zng-ui/zng/blob/main/examples/config/src/main.rs for
    // an example of how to split configs with an special key prefix to another file.
}
