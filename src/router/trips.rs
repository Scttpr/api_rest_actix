use actix_web::{web, Resource};
use crate::utils::crud::{
    get_handler,
    list_handler,
    create_handler,
    update_handler,
    delete_handler,
};
use crate::utils::constants::rights::trips::{
    CAN_CREATE,
    CAN_DELETE,
    CAN_GET,
    CAN_UPDATE,
};

pub fn list() -> Resource {
    web::resource("")
        //.wrap(middleware::resolve_rights(vec![CAN_GET]))
        .route(web::get().to(list_handler))
}

pub fn create() -> Resource {
    web::resource("")
        //.wrap(middleware::resolve_rights(vec![CAN_CREATE]))
        .route(web::post().to(create_handler))
}

pub fn get() -> Resource {
    web::resource("/{id}")
        //.wrap(middleware::resolve_rights(vec![CAN_GET]))
        .route(web::get().to(get_handler))
}

pub fn update() -> Resource {
    web::resource("/{id}")
        //.wrap(middleware::resolve_rights(vec![CAN_UPDATE]))
        .route(web::patch().to(update_handler))
}

pub fn delete() -> Resource {
    web::resource("/{id}")
        //.wrap(middleware::resolve_rights(vec![CAN_DELETE]))
        .route(web::delete().to(delete_handler))
}
