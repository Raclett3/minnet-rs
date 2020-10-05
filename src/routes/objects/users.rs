use crate::objects::activitypub::{
    json_ld::JsonLD,
    object::Object,
    person::{Person, PublicKey},
};
use crate::objects::context::Context;
use crate::objects::uri::{inbox_url, publickey_uri, users_uri};
use crate::{json_response, unwrap_result_or_500};
use actix_web::{web, HttpResponse};
use std::fs::read_to_string;
use std::sync::Mutex;

pub async fn users(
    web::Path(username): web::Path<String>,
    data: web::Data<Mutex<Context>>,
) -> HttpResponse {
    let user = "fuyuko";
    println!("User: {}", username);
    if username == user {
        let host = &unwrap_result_or_500!(data.lock()).config.host;
        let uri = users_uri(host, user);
        let key_string = unwrap_result_or_500!(read_to_string("public.pem"));
        let key = PublicKey::new(&publickey_uri(host, user), &uri, &key_string);
        let person = Person::new(&uri, Some(user), Some(&inbox_url(host)), Some(key));
        json_response!(&JsonLD::append_context(Object::Person(person)))
    } else {
        HttpResponse::NotFound().body("Not found")
    }
}
