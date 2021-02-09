use crate::utils::{context_with_connection, init_test_connection};
use actix_web::web;
use minnet_rs::routes::well_known::{webfinger, ResourceQuery};
use std::sync::Mutex;

#[actix_rt::test]
async fn test_webfinger() {
    let conn = init_test_connection();
    let context = web::Data::new(Mutex::new(context_with_connection(conn)));
    assert_eq!(
        webfinger(
            web::Query(ResourceQuery {
                resource: Some("acct:asahi@example.com".to_string()),
            }),
            context.clone()
        )
        .await
        .status()
        .as_u16(),
        200
    );
    assert_eq!(
        webfinger(
            web::Query(ResourceQuery {
                resource: Some("acct:fuyuko@example.com".to_string()),
            }),
            context.clone()
        )
        .await
        .status()
        .as_u16(),
        200
    );
    assert_eq!(
        webfinger(
            web::Query(ResourceQuery {
                resource: Some("acct:asahi@a.example.com".to_string()),
            }),
            context.clone()
        )
        .await
        .status()
        .as_u16(),
        404
    );
    assert_eq!(
        webfinger(
            web::Query(ResourceQuery {
                resource: Some("asahi@example.com".to_string()),
            }),
            context.clone()
        )
        .await
        .status()
        .as_u16(),
        404
    );
}
