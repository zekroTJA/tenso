use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct AuthUser {
    pub username: String,
    pub password_hash: String,
}
