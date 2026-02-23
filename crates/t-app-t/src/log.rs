//! Log setup.

use std::{fs, path::PathBuf};

use tracing_subscriber::{prelude::*, *};
use zng::text::{Txt, formatx};

// called by cli::run as soon as possible
pub fn init(
    filter: String,
    rotation: String,
    directory: Option<PathBuf>,
) -> Result<Option<PathBuf>, Txt> {
    // always print, good for debug and the crash-handler collects stdout/err.
    let log = registry().with(fmt::layer().without_time());

    // log filter from Zng (noisy dependencies)
    let zng_filter = tracing_subscriber::filter::FilterFn::new(|m| {
        if let Some(level) = tracing::level_filters::LevelFilter::current().into_level() {
            zng::app::print_tracing_filter(&level, m, &|_| true)
        } else {
            false
        }
    });

    // log filter from env/args
    let filter = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .parse_lossy(filter);

    if let Some(mut dir) = directory {
        // also append log file, with optional rolling frequency

        if let Ok(d) = dir.strip_prefix("{cache}") {
            dir = zng::env::cache(d)
        } else if let Ok(d) = dir.strip_prefix("{config}") {
            dir = zng::env::config(d)
        }
        let dir = dir;

        match fs::create_dir_all(&dir) {
            Ok(()) => {
                let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
                    .rotation(match rotation.as_str() {
                        "daily" => tracing_appender::rolling::Rotation::DAILY,
                        "hourly" => tracing_appender::rolling::Rotation::HOURLY,
                        "minutely" => tracing_appender::rolling::Rotation::MINUTELY,
                        "never" => tracing_appender::rolling::Rotation::NEVER,
                        u => {
                            eprintln!("unknown log rotation {u:?}, will never rotate");
                            tracing_appender::rolling::Rotation::NEVER
                        }
                    })
                    .filename_prefix("t-app-t")
                    .filename_suffix("log")
                    .build(&dir)
                    .unwrap();

                let write_log = fmt::layer().with_ansi(false).with_writer(file_appender);

                log.with(write_log).with(filter).with(zng_filter).init();

                Ok(Some(dir))
            }
            Err(e) => {
                log.with(filter).with(zng_filter).init();
                Err(formatx!("cannot log to `{}`, {e}", dir.display()))
            }
        }
    } else {
        // init with only printer
        log.with(filter).init();
        Ok(None)
    }
}
