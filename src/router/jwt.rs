use actix_web::{Resource, web, HttpResponse};

pub fn generate() -> Resource {
    web::resource("/jwt/generate")
        .route(web::post().to(|| {
            HttpResponse::Ok().body("Generate")
        }))
}

pub fn renew() -> Resource {
    web::resource("/jwt/renew")
        .route(web::post().to(|| {
            HttpResponse::Ok().body("Renew")
        }))
}

pub fn get_user() -> Resource {
    web::resource("/jwt/user")
        .route(web::get().to(|| {
            HttpResponse::Ok().body("get_user")
        }))
}
