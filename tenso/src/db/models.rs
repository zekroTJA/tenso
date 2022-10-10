use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Debug)]
pub struct AuthUser {
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable, Debug, Serialize, Clone)]
pub struct Link {
    pub id: String,
    pub ident: String,
    pub creator_id: String,
    pub created_date: NaiveDateTime,
    pub destination: String,
    pub enabled: bool,
    pub permanent_redirect: bool,
}

#[derive(Queryable, Debug, Serialize)]
pub struct StatEntry {
    pub id: String,
    pub link_id: String,
    pub created_date: NaiveDateTime,
    pub user_agent: Option<String>,
}

impl From<&Link> for StatEntry {
    fn from(link: &Link) -> Self {
        Self {
            created_date: Local::now().naive_local(),
            id: xid::new().to_string(),
            link_id: link.id.clone(),
            user_agent: None,
        }
    }
}
