mod nodeinfo;
mod objects;
mod well_known;

use actix_web::{web, Scope};

pub fn root() -> Scope {
    web::scope("/")
        .service(well_known::well_known())
        .service(nodeinfo::nodeinfo_route())
        .service(objects::objects())
}

#[macro_export]
macro_rules! json_response {
    ($obj:expr) => {
        if let Ok(json) = serde_json::to_string($obj) {
            actix_web::HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        } else {
            actix_web::HttpResponse::InternalServerError().body("Internal Server Error")
        }
    };
}

#[macro_export]
macro_rules! unwrap_option_or_500 {
    ($option:expr) => {
        if let Some(v) = $option {
            v
        } else {
            return actix_web::HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
}

#[macro_export]
macro_rules! unwrap_result_or_500 {
    ($result:expr) => {
        if let Ok(v) = $result {
            v
        } else {
            return actix_web::HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
}
