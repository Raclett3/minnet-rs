use crate::misc::random_id::random_id;
use crate::objects::models::users::User;
use crate::objects::models::users_auth::UserAuth;
use crate::schema::users::dsl as users;
use crate::schema::users_auth::dsl::users_auth;
use bcrypt::{hash, verify};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use openssl::rsa::Rsa;

pub trait UserController {
    fn sign_up(&self, username: &str, plain_password: &str) -> Option<User>;
    fn sign_in(&self, username: &str, plain_password: &str) -> bool;
}

impl UserController for PgConnection {
    fn sign_up(&self, username: &str, plain_password: &str) -> Option<User> {
        self.transaction(|| {
            let res = (|| {
                let count: i64 = users::users
                    .filter(users::username.eq(username))
                    .count()
                    .get_result(self)
                    .ok()?;

                if count != 0 {
                    return None;
                }

                let id = random_id();
                let username = username.to_lowercase();
                let encrypted = hash(plain_password, 10).ok()?;
                let key_pair = Rsa::generate(2048).ok()?;
                let public_key = String::from_utf8(key_pair.public_key_to_pem().ok()?).ok()?;
                let private_key = String::from_utf8(key_pair.private_key_to_pem().ok()?).ok()?;
                let auth = UserAuth::new(&username, &encrypted, &id, &public_key, &private_key);
                let user = User::new(&id, &username, None, &username, None, None, None);

                diesel::insert_into(users::users)
                    .values(&user)
                    .execute(self)
                    .ok()?;
                diesel::insert_into(users_auth)
                    .values(&auth)
                    .execute(self)
                    .ok()?;

                Some(user)
            })();
            res.ok_or(Error::RollbackTransaction)
        })
        .ok()
    }

    fn sign_in(&self, username: &str, plain_password: &str) -> bool {
        let username = username.to_lowercase();
        let auth = users_auth.find(&username).first::<UserAuth>(self);
        auth.ok()
            .and_then(
                |UserAuth {
                     encrypted_password, ..
                 }| verify(plain_password, &encrypted_password).ok(),
            )
            .unwrap_or(false)
    }
}
