pub fn users_uri(host: &str, name: &str) -> String {
    format!("https://{}/users/{}", host, name)
}

pub fn publickey_uri(host: &str, name: &str) -> String {
    format!("https://{}/publickey/{}", host, name)
}

pub fn inbox_url(host: &str) -> String {
    format!("https://{}/inbox", host)
}
