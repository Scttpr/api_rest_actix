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
use crate::utils::constants::LOG_PATH;

pub fn init() -> Result<(), SetLoggerError> {
    const STDERR: &str = "stderr";
    const LOGFILE: &str = "logfile";

    let file_path = format!("{}/api.log", env::get_var(LOG_PATH));

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build(LOGFILE, Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build(STDERR, Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender(LOGFILE)
                .appender(STDERR)
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    Ok(())
}
