#![allow(dead_code)]

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Connection;
use minnet_rs::objects::context;
use minnet_rs::objects::context::config::Config;
use minnet_rs::schema::{users::dsl::users, users_auth::dsl::users_auth};

pub fn init_test_connection() -> PgConnection {
    let config = Config::from_file("config/config.toml").expect("Failed to load config.toml");
    let database = &config.database;

    let url = format!(
        "postgres://{}:{}@{}/{}_test",
        database.user, database.password, database.host, database.database
    );

    let conn = PgConnection::establish(&url).unwrap();
    diesel::delete(users_auth).execute(&conn).unwrap();
    diesel::delete(users).execute(&conn).unwrap();

    conn
}

pub fn context_with_connection(conn: PgConnection) -> context::Context {
    context::Context {
        conn,
        config: context::config::Config {
            host: "example.com".to_string(),
            port: 8080,
            database: context::config::Database {
                host: "".to_string(),
                user: "".to_string(),
                password: "".to_string(),
                database: "".to_string(),
            },
        },
    }
}
