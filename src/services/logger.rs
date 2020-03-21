use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use crate::services::env;
use crate::utils::constants::{LOG_LEVEL, LOG_PATH, APP_ENV};

const TRACE_LEVEL: &str = "trace";
const DEBUG_LEVEL: &str = "debug";
const INFO_LEVEL: &str = "info";
const WARN_LEVEL: &str = "warn";
const ERROR_LEVEL: &str ="error";

const STDERR: &str = "stderr";
const LOGFILE: &str = "logfile";

pub fn init() -> Result<(), SetLoggerError> {
    let level: LevelFilter = match &*env::get_var(LOG_LEVEL) {
        TRACE_LEVEL => LevelFilter::Trace,
        DEBUG_LEVEL => LevelFilter::Debug,
        INFO_LEVEL => LevelFilter::Info,
        WARN_LEVEL => LevelFilter::Warn,
        ERROR_LEVEL => LevelFilter::Error,
        _ => LevelFilter::Trace,
    };

    let log_path = format!("{}/api.log", env::get_var(LOG_PATH));

    let stderr: ConsoleAppender = ConsoleAppender::builder().target(Target::Stderr).build();

    let log_file: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(log_path)
        .unwrap();

    let config = Config::builder()
        .appender(  Appender::builder()
            .build(LOGFILE, Box::new(log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build(STDERR, Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender(LOGFILE)
                .appender(STDERR)
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    Ok(())
}
