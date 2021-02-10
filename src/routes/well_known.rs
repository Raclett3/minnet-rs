use crate::objects::context::Context;
use crate::objects::others::well_known::*;
use crate::objects::uri::users_uri;
use crate::unwrap_result_or_500;
use actix_web::{web, HttpResponse, Scope};
use regex::Regex;
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
        let pattern = format!("^acct:([a-zA-Z0-9]+)@{}$", host);
        let re = unwrap_result_or_500!(Regex::new(&pattern));
        let matched = re
            .captures(&resource)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str().to_lowercase());

        if let Some(username) = matched {
            println!("Webfinger: {}", resource);
            let subject = format!("acct:{}@{}", username, host);
            let href = users_uri(host, &username);
            let links = vec![Link::new(
                "self".to_string(),
                "application/activity+json".to_string(),
                href,
            )];
            let user = Webfinger::new(Some(subject), links);
            return HttpResponse::Ok().json(user);
        }
    }

    HttpResponse::NotFound()
        .content_type("application/json")
        .body("{}")
}

async fn nodeinfo(data: web::Data<Mutex<Context>>) -> HttpResponse {
    let host = &unwrap_result_or_500!(data.lock()).config.host;
    let links = vec![Link::new(
        "http://nodeinfo.diaspora.software/ns/schema/2.0".to_string(),
        "application/json".to_string(),
        format!("https://{}/nodeinfo/2.0", host),
    )];
    let nodeinfo = Webfinger::new(None, links);
    HttpResponse::Ok().json(nodeinfo)
}

pub fn well_known() -> Scope {
    web::scope("/.well-known")
        .service(web::resource("/webfinger").route(web::get().to(webfinger)))
        .service(web::resource("/nodeinfo").route(web::get().to(nodeinfo)))
}
