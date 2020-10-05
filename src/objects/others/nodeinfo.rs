use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Software {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub version: String,
    pub software: Software,
    pub protocols: Vec<String>,
    pub open_registrations: bool,
}

impl NodeInfo {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            version: "2.0".to_string(),
            software: Software {
                name: name.to_string(),
                version: version.to_string(),
            },
            protocols: vec!["activitypub".to_string()],
            open_registrations: true,
        }
    }
}
