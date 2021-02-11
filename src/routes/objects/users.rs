use crate::objects::activitypub::{
    json_ld::JsonLD,
    object::Object,
    person::{Person, PublicKey},
};
use crate::objects::context::Context;
use crate::objects::models::users_auth::UserAuth;
use crate::objects::uri::{inbox_url, publickey_uri, users_uri};
use crate::schema::users_auth::dsl::users_auth;
use crate::unwrap_result_or_500;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use std::sync::Mutex;

pub async fn users(
    web::Path(username): web::Path<String>,
    data: web::Data<Mutex<Context>>,
) -> HttpResponse {
    let context = unwrap_result_or_500!(data.lock());
    println!("User: {}", username);
    let conn = &context.conn;
    let auth = users_auth.find(&username).first::<UserAuth>(conn);
    if let Ok(auth) = auth {
        let host = &context.config.host;
        let uri = users_uri(host, &username);
        let key = PublicKey::new(&publickey_uri(host, &username), &uri, &auth.public_key);
        let person = Person::new(&uri, Some(&username), Some(&inbox_url(host)), Some(key));
        HttpResponse::Ok().json(JsonLD::append_context(Object::Person(person)))
    } else {
        HttpResponse::NotFound().body("Not found")
    }
}
