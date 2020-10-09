use rand::prelude::*;

const ID_CHARS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

pub fn random_id() -> String {
    (0..16)
        .map(|_| {
            let index = random::<usize>() % 36usize;
            ID_CHARS[index] as char
        })
        .collect()
}
