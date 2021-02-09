use crate::objects::context::Context;
use crate::objects::others::well_known::*;
use crate::objects::uri::users_uri;
use crate::{json_response, unwrap_result_or_500};
use actix_web::{web, HttpResponse, Scope};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct ResourceQuery {
    pub resource: Option<String>,
}

pub async fn webfinger(
    query: web::Query<ResourceQuery>,
    data: web::Data<Mutex<Context>>,
) -> HttpResponse {
    let host = &unwrap_result_or_500!(data.lock()).config.host;
    if let Some(resource) = &query.resource {
        println!("Webfinger: {}", resource);
        let username = "fuyuko";
        let subject = format!("acct:{}@{}", username, host);
        let href = users_uri(host, username);
        let links = vec![Link::new(
            "self".to_string(),
            "application/activity+json".to_string(),
            href,
        )];
        let user = Webfinger::new(Some(subject), links);
        json_response!(&user)
    } else {
        HttpResponse::NotFound().body("Query not present")
    }
}

async fn nodeinfo(data: web::Data<Mutex<Context>>) -> HttpResponse {
    let host = &unwrap_result_or_500!(data.lock()).config.host;
    let links = vec![Link::new(
        "http://nodeinfo.diaspora.software/ns/schema/2.0".to_string(),
        "application/json".to_string(),
        format!("https://{}/nodeinfo/2.0", host),
    )];
    let nodeinfo = Webfinger::new(None, links);
    json_response!(&nodeinfo)
}

pub fn well_known() -> Scope {
    web::scope("/.well-known")
        .service(web::resource("/webfinger").route(web::get().to(webfinger)))
        .service(web::resource("/nodeinfo").route(web::get().to(nodeinfo)))
}
