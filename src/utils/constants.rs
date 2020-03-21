pub const HOST: &str = "HOST";
pub const PORT: &str = "PORT";
pub const VERSION: &str = "VERSION";
pub const APP_ENV: &str = "APP_ENV";
pub const LOG_PATH: &str = "LOG_PATH";
pub const LOG_LEVEL: &str = "LOG_LEVEL";

// @todo maybe move to rights.rs
pub mod rights {
    pub mod trips {
        pub const CAN_GET: &str = "CAN_GET_TRIPS";
        pub const CAN_CREATE: &str = "CAN_CREATE_TRIPS";
        pub const CAN_UPDATE: &str = "CAN_UPDATE_TRIPS";
        pub const CAN_DELETE: &str = "CAN_DELETE_TRIPS";

        pub const CAN_SELF_GET: &str = "CAN_SELF_GET_TRIPS";
        pub const CAN_SELF_UPDATE: &str = "CAN_SELF_LIST_TRIPS";
        pub const CAN_SELF_DELETE: &str = "CAN_SELF_LIST_TRIPS";
    }
}
