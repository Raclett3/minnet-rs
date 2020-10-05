use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub rel: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub href: String,
}

impl Link {
    pub fn new(rel: String, link_type: String, href: String) -> Self {
        Self {
            rel,
            link_type,
            href,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Webfinger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub links: Vec<Link>,
}

impl Webfinger {
    pub fn new(subject: Option<String>, links: Vec<Link>) -> Self {
        Self { subject, links }
    }
}
