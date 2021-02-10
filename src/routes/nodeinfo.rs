use crate::objects::others::nodeinfo::NodeInfo;
use actix_web::{web, HttpResponse, Resource};

async fn nodeinfo() -> HttpResponse {
    HttpResponse::Ok().json(NodeInfo::new("minnet", "0.0.0"))
}

pub fn nodeinfo_route() -> Resource {
    web::resource("/nodeinfo/2.0").route(web::get().to(nodeinfo))
}
