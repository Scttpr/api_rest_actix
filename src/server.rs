use crate::services::{logger, env};
use crate::utils::constants::{HOST, PORT};
use crate::router;
use actix_web::{HttpServer, App};

pub async fn init_server() -> std::io::Result<()> {
    env::load_dotenv();
    logger::init();

    let app_url: String = format!("{}:{}", env::get_var(HOST), env::get_var(PORT));

    HttpServer::new(|| {
        App::new()
            .configure(router::get_routes)
    })
        .bind(&app_url)?
        .run()
        .await
}
