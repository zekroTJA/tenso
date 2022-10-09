use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct AuthUser {
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable, Debug)]
pub struct Link {
    pub id: String,
    pub ident: String,
    pub creator_id: String,
    pub created_date: NaiveDateTime,
    pub destination: String,
    pub enabled: bool,
    pub permanent_redirect: bool,
}

#[derive(Queryable, Debug)]
pub struct StatEntry {
    pub id: String,
    pub link_id: String,
    pub created_date: NaiveDateTime,
    pub user_agent: String,
}
