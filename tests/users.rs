mod utils;

use minnet_rs::controllers::users::UserController;

#[test]
fn test_users() {
    let conn = utils::init_test_connection();
    assert!(conn.sign_up("asahi", "asahi_password").is_some());
    assert!(conn.sign_up("asahi", "asahi_password").is_none());
    assert!(conn.sign_up("Asahi", "asahi_password").is_none());
    assert!(conn.sign_up("fuyuko", "fuyuko_password").is_some());

    assert!(conn.sign_in("asahi", "asahi_password"));
    assert!(conn.sign_in("Asahi", "asahi_password"));
    assert!(conn.sign_in("fuyuko", "fuyuko_password"));
    assert!(!conn.sign_in("asahi", "ASAHI_PASSWORD"));
    assert!(!conn.sign_in("asahi", "fuyuko_password"));
    assert!(!conn.sign_in("mei", "fuyuko_password"));
}
