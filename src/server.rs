use actix_web::{HttpServer, App, middleware};

use crate::services::{logger, env};
use crate::utils::constants::{HOST, PORT};
use crate::router;

pub async fn init_server() -> std::io::Result<()> {
    env::load_dotenv();
    logger::init().expect("Failed to init logger");
    let app_url: String = format!("{}:{}", env::get_var(HOST), env::get_var(PORT));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(router::get_routes)
    })
        .bind(&app_url)?
        .run()
        .await
}
