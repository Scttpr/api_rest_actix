use actix_web::{HttpResponse, HttpRequest};
use log;

pub fn create_handler() -> HttpResponse {
    HttpResponse::Ok().body("Create")
}

pub fn delete_handler() -> HttpResponse {
    HttpResponse::Ok().body("Delete")
}

pub fn get_handler(req: HttpRequest) -> HttpResponse {
    log::info!("From get handler");
    HttpResponse::Ok().body("Get")
}

pub fn list_handler() -> HttpResponse {
    HttpResponse::Ok().body("List")
}

pub fn update_handler() -> HttpResponse {
    HttpResponse::Ok().body("Update")
}
