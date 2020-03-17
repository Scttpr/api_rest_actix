use std::env;
use crate::utils::errors::error_messages;

pub fn load_dotenv() {
    dotenv::dotenv().expect("Failed to read .env file");
}

pub fn get_var(name: &str) -> String {
    env::var(&name).expect(&error_messages::not_found(name))
}
