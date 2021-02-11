use std::ops::RangeBounds;

fn validate<F: Fn(char) -> bool, R: RangeBounds<usize>>(
    target: &str,
    validator: F,
    len_range: R,
) -> bool {
    len_range.contains(&target.len()) && target.chars().all(validator)
}

pub fn validate_username(username: &str) -> bool {
    validate(
        username,
        |ch| ch.is_ascii_alphanumeric() || ch == '_',
        1..=32,
    )
}

pub fn validate_password(password: &str) -> bool {
    validate(password, |ch| ch.is_ascii() && !ch.is_control(), 8..=128)
}
