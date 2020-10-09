pub mod config;

use diesel::pg::PgConnection;

pub struct Context {
    pub config: config::Config,
    pub conn: PgConnection,
}
