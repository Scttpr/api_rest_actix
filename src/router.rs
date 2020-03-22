mod trips;
use actix_web::web;

pub fn get_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Trips
            .service(
                web::scope("/trips")
                    .service(trips::list())
                    .service(trips::get())
                    .service(trips::create())
                    .service(trips::update())
                    .service(trips::delete())
            )
    );
}
