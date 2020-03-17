// @todo check/create Http Errors
pub mod error_messages {
    pub fn not_found (element: &str) -> String {
        let message = String::from(element) + " not found";
        message
    }
}
