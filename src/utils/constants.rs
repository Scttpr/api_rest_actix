pub const HOST: &str = "HOST";
pub const PORT: &str = "PORT";
pub const VERSION: &str = "VERSION";

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
