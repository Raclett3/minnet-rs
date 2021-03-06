#[macro_use]
extern crate diesel;

mod controllers;
mod misc;
mod objects;
mod routes;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::Connection;
use objects::context::{config::*, Context};
use std::sync::Mutex;

use routes::root;

fn connection(database_url: &str) -> Option<PgConnection> {
    PgConnection::establish(&database_url).ok()
}

fn database_url(database: &Database) -> String {
    format!(
        "postgres://{}:{}@{}/{}",
        database.user, database.password, database.host, database.database
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("config/config.toml").expect("Failed to load config.toml");
    let addr = format!("127.0.0.1:{}", config.port);

    let url = database_url(&config.database);
    let conn = connection(&url).expect("Failed to establish ");

    let data = web::Data::new(Mutex::new(Context { config, conn }));
    HttpServer::new(move || App::new().app_data(data.clone()).service(root()))
        .bind(addr)?
        .run()
        .await
}
