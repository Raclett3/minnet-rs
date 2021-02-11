use minnet_rs::misc;

#[test]
fn test_random_id() {
    use misc::random_id::random_id;
    use std::collections::HashSet;

    let mut set = HashSet::new();

    for _ in 0..10000 {
        let id = random_id();
        assert!(!set.contains(&id));
        set.insert(id);
    }
}

#[test]
fn test_validator() {
    use misc::validators::{validate_password, validate_username};

    assert!(validate_password("password"));
    assert!(validate_password("abcd1234"));
    assert!(validate_password("What's up bro ;)"));
    assert!(!validate_password("abc123")); // too short
    assert!(!validate_password("日本語のパスワード")); // contains non-ascii chars

    assert!(validate_username("asahi"));
    assert!(validate_username("fuyuko"));
    assert!(validate_username("abc_123"));
    assert!(!validate_username(
        "this_is_too_long_username_and_this_is_invalid"
    )); // too long
    assert!(!validate_username("日本語のユーザー名")); // includes non-ascii chars
    assert!(!validate_username("What?")); // includes invalid chars
}
