table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        host -> Nullable<Varchar>,
        nickname -> Varchar,
        uri -> Nullable<Varchar>,
        inbox -> Nullable<Varchar>,
        public_key -> Nullable<Varchar>,
    }
}

table! {
    users_auth (username) {
        username -> Varchar,
        encrypted_password -> Varchar,
        users_id -> Varchar,
        private_key -> Varchar,
        public_key -> Varchar,
    }
}

joinable!(users_auth -> users (users_id));

allow_tables_to_appear_in_same_query!(
    users,
    users_auth,
);
