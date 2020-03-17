mod server;
mod services;
mod utils;
mod middlewares;
mod router;

use server::init_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_server().await
}
