//! Log setup.

use std::{fs, path::PathBuf};

use tracing_subscriber::{prelude::*, *};
use zng::text::{formatx, Txt};

// called by cli::run as soon as possible
pub fn init(filter: String, rotation: String, directory: String) -> Result<Option<PathBuf>, Txt> {
    // always print, good for debug and the crash-handler collects stdout/err.
    let log = registry().with(fmt::layer().without_time());

    // log filter from env/args
    let filter = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .parse_lossy(&filter);

    let directory = if directory.is_empty() {
        Some(if directory.starts_with("{cache}") {
            zng::env::cache(directory["{cache}".len()..].trim_start_matches('/'))
        } else if directory.starts_with("{config}") {
            zng::env::config(directory["{cache}".len()..].trim_start_matches('/'))
        } else {
            PathBuf::from(directory)
        })
    } else {
        None
    };

    if let Some(dir) = directory {
        // also append log file, with optional rolling frequency

        match fs::create_dir_all(&dir) {
            Ok(()) => {
                let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
                    .rotation(match rotation.as_str() {
                        "daily" => tracing_appender::rolling::Rotation::DAILY,
                        "hourly" => tracing_appender::rolling::Rotation::HOURLY,
                        "minutely" => tracing_appender::rolling::Rotation::MINUTELY,
                        "never" => tracing_appender::rolling::Rotation::NEVER,
                        _ => unreachable!(),
                    })
                    .filename_prefix("t-app-t")
                    .filename_suffix("log")
                    .build(&dir)
                    .unwrap();

                let write_log = fmt::layer().with_ansi(false).with_writer(file_appender);

                log.with(write_log).with(filter).init();

                Ok(Some(dir))
            }
            Err(e) => {
                log.with(filter).init();
                Err(formatx!("cannot log to `{}`, {e}", dir.display()))
            }
        }
    } else {
        // init with only printer
        log.with(filter).init();
        Ok(None)
    }
}
