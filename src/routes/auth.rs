use crate::controllers::users::UserController;
use crate::misc::validators::{validate_password, validate_username};
use crate::objects::context::Context;
use crate::unwrap_result_or_500;
use actix_web::{web, HttpResponse, Scope};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct Auth {
    pub username: Option<String>,
    pub password: Option<String>,
}

pub async fn sign_up(
    web::Json(Auth { username, password }): web::Json<Auth>,
    data: web::Data<Mutex<Context>>,
) -> HttpResponse {
    let (username, password) = match (username, password) {
        (Some(username), Some(password))
            if validate_username(&username) && validate_password(&password) =>
        {
            (username, password)
        }
        _ => return HttpResponse::BadRequest().body("{}"),
    };

    let context = unwrap_result_or_500!(data.lock());
    let conn = &context.conn;
    if conn.sign_up(&username, &password).is_some() {
        HttpResponse::Created().body("{}")
    } else {
        HttpResponse::Conflict().body("{}")
    }
}

pub fn auth() -> Scope {
    web::scope("/auth").service(web::resource("/signup").route(web::post().to(sign_up)))
}
