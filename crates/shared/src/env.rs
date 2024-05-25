use std::path::{Path, PathBuf};
use zng::{
    hot_reload::{lazy_static, lazy_static_init},
    l10n::Langs,
};

pub struct TtAppTtArgs {
    /// Paths the user attempted to open the app with.
    pub paths: Vec<PathBuf>,

    /// Log dir.
    ///
    /// Place files to include in crash reports here.
    ///
    /// Is `None` if log writing is disabled.
    pub log_dir: Option<PathBuf>,

    /// view_process::run_same_process
    pub no_view_process: bool,

    pub no_crash_handler: bool,

    /// Preferred initial language.
    pub lang: Langs,
    /// Localization resources.
    pub lang_dir: PathBuf,
}

/// {{app}} parsed startup args.
pub fn args() -> &'static TtAppTtArgs {
    &CFG
}

impl TtAppTtArgs {
    /// Get a path in log dir, if log writing is enabled.
    pub fn log(&self, relative_path: impl AsRef<Path>) -> Option<PathBuf> {
        self.log_dir.as_ref().map(|p| p.join(relative_path))
    }
}

// called by t-app-t/cli.rs
pub fn init_args(cfg: TtAppTtArgs) {
    if lazy_static_init(&CFG, cfg).is_err() {
        panic!("shared::env::args is already inited");
    }
}
lazy_static! {
    static ref CFG: TtAppTtArgs =
        panic!("shared::env::args not inited, only use after t_app_t::cli");
}
