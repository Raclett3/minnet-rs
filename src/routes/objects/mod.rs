mod users;

use actix_web::{web, Scope};

pub fn objects() -> Scope {
    web::scope("/").service(web::resource("/users/{username}").route(web::get().to(users::users)))
}
