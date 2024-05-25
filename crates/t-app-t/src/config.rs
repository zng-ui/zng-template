use zng::config::*;

// others formats are available as Cargo features
type FileConfig = JsonConfig;
const FILE: &str = "config.json";

/// Initialize user config in the app context.
pub fn app_init() {
    let app = FileConfig::sync(zng::env::res(FILE));
    let app = ReadOnlyConfig::new(app);
    let user = FileConfig::sync(zng::env::config(FILE));
    let config = FallbackConfig::new(user, app);
    CONFIG.load(config);
}
