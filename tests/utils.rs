use diesel::pg::PgConnection;
use diesel::Connection;
use minnet_rs::objects::context::config::Config;
use minnet_rs::schema::{
    users::dsl::users,
    users_auth::dsl::users_auth,
};
use diesel::prelude::*;

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
