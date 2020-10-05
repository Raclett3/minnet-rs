mod objects;
mod routes;

use actix_web::{web, App, HttpServer};
use objects::context::{config::Config, Context};
use std::sync::Mutex;

use routes::root;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("config/config.toml").expect("Failed to load config.toml");
    let data = web::Data::new(Mutex::new(Context { config }));
    HttpServer::new(move || App::new().app_data(data.clone()).service(root()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
