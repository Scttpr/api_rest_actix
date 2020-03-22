use actix_web::{web, Resource};
use crate::utils::crud::{
    get_handler,
    list_handler,
    create_handler,
    update_handler,
    delete_handler,
};

pub fn list() -> Resource {
    web::resource("")
        .route(web::get().to(list_handler))
}

pub fn create() -> Resource {
    web::resource("")
        .route(web::post().to(create_handler))
}

pub fn get() -> Resource {
    web::resource("/{id}")
        .route(web::get().to(get_handler))
}

pub fn update() -> Resource {
    web::resource("/{id}")
        .route(web::patch().to(update_handler))
}

pub fn delete() -> Resource {
    web::resource("/{id}")
        .route(web::delete().to(delete_handler))
}
