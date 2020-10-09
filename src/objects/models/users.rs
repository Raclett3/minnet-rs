use crate::schema::users as users_table;

#[derive(Queryable, Insertable)]
#[table_name = "users_table"]
pub struct User {
    pub id: String,
    pub username: String,
    pub host: Option<String>,
    pub nickname: String,
    pub uri: Option<String>,
    pub inbox: Option<String>,
    pub public_key: Option<String>,
}

impl User {
    pub fn new(
        id: &str,
        username: &str,
        host: Option<&str>,
        nickname: &str,
        uri: Option<&str>,
        inbox: Option<&str>,
        public_key: Option<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            username: username.to_string(),
            host: host.map(str::to_string),
            nickname: nickname.to_string(),
            uri: uri.map(str::to_string),
            inbox: inbox.map(str::to_string),
            public_key: public_key.map(str::to_string),
        }
    }
}
