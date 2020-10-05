use super::object::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonLD {
    #[serde(rename = "@context")]
    pub context: Vec<String>,

    #[serde(flatten)]
    pub object: Object,
}

impl JsonLD {
    pub fn new(context: Vec<&str>, object: Object) -> Self {
        Self {
            context: context.iter().copied().map(str::to_string).collect(),
            object,
        }
    }

    pub fn append_context(object: Object) -> Self {
        let context = vec![
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ];
        JsonLD::new(context, object)
    }
}
