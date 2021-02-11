use crate::utils::{context_with_connection, init_test_connection};
use actix_web::web;
use minnet_rs::controllers::users::UserController;
use minnet_rs::routes::objects::users::users;
use std::sync::Mutex;

#[actix_rt::test]
async fn test_users() {
    let conn = init_test_connection();
    conn.sign_up("asahi", "asahi_password");
    let context = web::Data::new(Mutex::new(context_with_connection(conn)));
    assert_eq!(
        users(web::Path("asahi".to_string()), context.clone())
            .await
            .status()
            .as_u16(),
        200
    );
    assert_eq!(
        users(web::Path("fuyuko".to_string()), context.clone())
            .await
            .status()
            .as_u16(),
        404
    );
}
