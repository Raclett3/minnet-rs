use minnet_rs::misc::random_id::random_id;
use std::collections::HashSet;

#[test]
fn test_random_id() {
    let mut set = HashSet::new();

    for _ in 0..10000 {
        let id = random_id();
        assert!(!set.contains(&id));
        set.insert(id);
    }
}
