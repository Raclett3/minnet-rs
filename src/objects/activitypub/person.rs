use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TypePerson {
    Person,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

impl PublicKey {
    pub fn new(id: &str, owner: &str, public_key_pem: &str) -> Self {
        Self {
            id: id.to_string(),
            owner: owner.to_string(),
            public_key_pem: public_key_pem.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,

    #[serde(rename = "type")]
    pub object_type: TypePerson,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub preferred_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inbox: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub public_key: Option<PublicKey>,
}

impl Person {
    pub fn new(
        id: &str,
        preferred_username: Option<&str>,
        inbox: Option<&str>,
        public_key: Option<PublicKey>,
    ) -> Self {
        Self {
            id: id.to_string(),
            object_type: TypePerson::Person,
            preferred_username: preferred_username.map(str::to_string),
            inbox: inbox.map(str::to_string),
            public_key,
        }
    }
}
