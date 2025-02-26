use std::{fs, io, path::PathBuf};

use clap::*;
use zng::text::{Txt, formatx};

// called on `zng::env::init!`
zng::env::on_process_start!(|_| {
    run();
});

/// {{app}} command line interface (CLI)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Open {{app}}
    #[arg(num_args(0..))]
    paths: Vec<PathBuf>, // avoids clap errors if the user attempts to open files

    /// Saves the configs associated with [env: VAR] as the new default next run.
    ///
    /// The app does not run with this flag, it just saves and closes.
    #[arg(long, action)]
    env_save: bool,

    /// Clear saved env configs.
    #[arg(long, action)]
    env_reset: bool,

    /// Move config files to new path.
    #[arg(long)]
    config_migrate: Option<PathBuf>,

    /// Remove cache files.
    #[arg(long, action)]
    cache_clear: bool,

    /// Move cache files to new path.
    #[arg(long)]
    cache_migrate: Option<PathBuf>,

    /// Log filter
    ///
    /// Can be a global verbosity level:
    ///
    /// [trace, debug, info, warn, error]
    ///
    /// Or targeted filters:
    ///
    /// [target[span{field=value}]=level][,...]
    #[clap(
        long,
        env="T_APP_T_LOG",
        default_value_t = {"info".to_owned()},
        value_names = &["FILTER"],
    )]
    pub log: String,

    /// Log files directory
    ///
    /// Set to empty to not save logs
    #[clap(long, env = "T_APP_T_LOG_DIR", value_names = &["DIR"], default_value = "{cache}/log")]
    pub log_dir: Option<PathBuf>,

    /// Log file rotation rolling frequency
    #[clap(
        long,
        env="T_APP_T_LOG_ROTATION",
        default_value = "daily",
        value_parser = builder::PossibleValuesParser::new(["daily", "hourly", "minutely", "never"]),
        value_names = &["ROTATION"],
    )]
    pub log_rotation: String,

    /// Run the system integration and renderer in the same process
    ///
    /// In this mode the app will not recover from driver related crashes.
    #[clap(long, env = "T_APP_T_NO_VIEW_PROCESS", action)]
    pub no_view_process: bool,

    /// Don't handle app crashes
    ///
    /// In this mode crashes are handled directly by the OS or attached debugger.
    #[clap(long, env = "T_APP_T_NO_CRASH_HANDLER", action)]
    pub no_crash_handler: bool,

    /// Initial language
    ///
    /// Value must be an Unicode Language Identifier, examples: "en-US", "zh-Hans, en".
    ///
    /// Is the system language by default.
    #[clap(long, env = "T_APP_T_LANG", default_value = "")]
    pub lang: zng::l10n::Langs,

    /// Localization files dir.
    #[clap(long, env = "T_APP_T_LANG_DIR", value_names = &["DIR"], default_value = "{res}/l10n")]
    pub lang_dir: Option<PathBuf>,
}

/// Runs CLI.
///
/// Initializes [`shared::env::cfg`].
///
/// Exits process if CLI only flags are set.
fn run() {
    // init saved env (and .env in dev builds)
    let dotenv_init_result = dotenv_init();
    if let Err(e) = &dotenv_init_result {
        eprintln!("{e}");
    }

    // parse args and env
    let matches = Cli::command().get_matches();
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    // start logging
    let log_dir = match crate::log::init(cli.log, cli.log_rotation, cli.log_dir) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            tracing::error!("{e}");
            None
        }
    };
    if let Err(e) = dotenv_init_result {
        tracing::error!("{e}");
    }

    // resolve localization resources
    let mut lang_dir = cli.lang_dir;
    if let Some(lang) = &mut lang_dir {
        if let Ok(d) = lang.strip_prefix("{res}") {
            *lang = zng::env::res(d)
        } else if let Ok(d) = lang.strip_prefix("{config}") {
            *lang = zng::env::config(d)
        }
    }
    let lang_dir = lang_dir.unwrap_or_else(|| zng::env::res("l10n"));

    // if args are just for saving..
    if cli.env_save {
        run_env_save(matches);
        zng::env::exit(0);
    } else if cli.env_reset {
        run_env_reset();
        zng::env::exit(0);
    }

    let mut is_cli_only_run = false;

    if cli.cache_clear {
        run_cache_clear();
        is_cli_only_run = true;
    }
    if let Some(p) = cli.cache_migrate {
        // writes new cache dir to config
        run_migrate_cache(p);
        is_cli_only_run = true;
    }

    if let Some(p) = cli.config_migrate {
        run_migrate_config(p);
        is_cli_only_run = true;
    }

    if is_cli_only_run {
        zng::env::exit(0);
    }

    shared::env::init_args(shared::env::TtAppTtArgs {
        paths: cli.paths,
        log_dir,
        no_view_process: cli.no_view_process,
        no_crash_handler: cli.no_crash_handler,
        lang: cli.lang,
        lang_dir,
    })
}

const ENV_SAVE_FILE: &str = "env-save.env";

fn dotenv_init() -> Result<(), Txt> {
    // .env only sets unset vars, so we try the "overwrite" first

    #[cfg(feature = "dev")]
    match dotenv::dotenv() {
        Ok(p) => println!("using {}", p.display()),
        Err(e) => {
            if !e.not_found() {
                return Err(formatx!("error loading `.env`, {e}"));
            }
        }
    }

    let path = zng::env::config(ENV_SAVE_FILE);
    if let Err(e) = dotenv::from_path(&path) {
        if !e.not_found() {
            return Err(formatx!("error reading `{}`, {e}", path.display()));
        }
    }

    let path = zng::env::res(ENV_SAVE_FILE);
    if let Err(e) = dotenv::from_path(&path) {
        if !e.not_found() {
            return Err(formatx!("error reading `{}`, {e}", path.display()));
        }
    }

    Ok(())
}

fn run_env_save(matches: ArgMatches) {
    let path = zng::env::config(ENV_SAVE_FILE);

    let mut s = format!(
        "# saved by {} --env-save",
        std::env::current_exe().unwrap().display()
    );

    for arg in Cli::command().get_arguments() {
        if let Some(env) = arg.get_env() {
            let id = arg.get_id().as_str();
            let env = env.to_string_lossy();

            if let Some(v) = matches.get_one::<String>(id) {
                s.push_str(&env);
                s.push('=');
                s.push_str(v);
            }
        }
    }

    match fs::write(path, s.as_bytes()) {
        Ok(_) => println!("saved"),
        Err(e) => eprintln!("{e}"),
    }
}

fn run_env_reset() {
    if let Err(e) = fs::remove_file(zng::env::config(ENV_SAVE_FILE)) {
        if e.kind() != io::ErrorKind::NotFound {
            eprintln!("failed to reset env save, {e}");
            return;
        }
    }
    println!("env save reset")
}

fn run_migrate_config(to: PathBuf) {
    if let Err(e) = zng::env::migrate_config(to) {
        eprintln!("failed to migrate config, {e}");
    } else {
        println!("config migrated")
    }
}

fn run_migrate_cache(to: PathBuf) {
    if let Err(e) = zng::env::migrate_cache(to) {
        eprintln!("failed to migrate cache, {e}");
    } else {
        println!("config migrated")
    }
}

fn run_cache_clear() {
    if let Err(e) = zng::env::clear_cache() {
        eprintln!("failed to fully clear cache, {e}");
    } else {
        println!("cache cleared")
    }
}
