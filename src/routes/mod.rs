pub mod nodeinfo;
pub mod objects;
pub mod well_known;

use actix_web::{web, Scope};

pub fn root() -> Scope {
    web::scope("/")
        .service(well_known::well_known())
        .service(nodeinfo::nodeinfo_route())
        .service(objects::objects())
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
