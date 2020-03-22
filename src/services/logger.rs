pub const LOG_LEVEL_INFO: &str = "info";
pub const LOG_LEVEL_WARN: &str = "warn";
pub const LOG_LEVEL_ERROR: &str = "error";

#[macro_export]
macro_rules! log {
    ($level:expr => $content:expr) => {
        use log::{info, warn, error};
        use std::fs::{OpenOptions, File, create_dir_all};
        use std::io::{Write, Error};

        use crate::services::env;
        use crate::utils::constants::{LOG_PATH, APP_ENV};
        use crate::services::logger::{LOG_LEVEL_WARN, LOG_LEVEL_ERROR};

        match $level {
            LOG_LEVEL_WARN => warn!("{}", $content),
            LOG_LEVEL_ERROR => error!("{}", $content),
            _ => info!("{}", $content)
        }

        if env::get_var(APP_ENV) != "prod" {
            let log_content: String = format!("{}\n", $content);
            let log_dir: String = env::get_var(LOG_PATH);
            let log_path: String = format!("{}/api.log", log_dir);

            create_dir_all(log_dir)?;

             let mut log_file: File = OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_path)?;

            log_file.write_all(log_content.as_bytes())?;
        }
    };
}


