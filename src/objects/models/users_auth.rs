use crate::schema::users_auth;

#[derive(Queryable, Insertable)]
#[table_name = "users_auth"]
pub struct UserAuth {
    pub username: String,
    pub encrypted_password: String,
    pub users_id: String,
    pub private_key: String,
    pub public_key: String,
}

impl UserAuth {
    pub fn new(
        username: &str,
        encrypted_password: &str,
        users_id: &str,
        private_key: &str,
        public_key: &str,
    ) -> Self {
        Self {
            username: username.to_string(),
            encrypted_password: encrypted_password.to_string(),
            users_id: users_id.to_string(),
            private_key: private_key.to_string(),
            public_key: public_key.to_string(),
        }
    }
}
