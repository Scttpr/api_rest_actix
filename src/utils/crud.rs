use actix_web::{HttpResponse};
use crate::utils::crud;

pub fn create_handler() -> HttpResponse {
    HttpResponse::Ok().body("Create")
}

pub fn delete_handler() -> HttpResponse {
    HttpResponse::Ok().body("Delete")
}

pub fn get_handler() -> HttpResponse {
    HttpResponse::Ok().body("Get")
}

pub fn list_handler() -> HttpResponse {
    HttpResponse::Ok().body("List")
}

pub fn update_handler() -> HttpResponse {
    HttpResponse::Ok().body("Update")
}
