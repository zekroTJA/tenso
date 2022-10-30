use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthCheckResponseModel {
    pub initialized: bool,
}

#[derive(Serialize)]
pub struct CountResponseModel<T: Into<usize>> {
    pub count: T,
}

#[derive(Deserialize)]
pub struct AuthInitRequestModel {
    pub username: String,
    pub password: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthLoginRequestModel {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LinkCreateRequestModel {
    pub ident: String,
    pub destination: String,
    pub enabled: bool,
    pub permanent_redirect: bool,
}

#[derive(Deserialize)]
pub struct LinkUpdateRequestModel {
    pub ident: Option<String>,
    pub destination: Option<String>,
    pub enabled: Option<bool>,
    pub permanent_redirect: Option<bool>,
}

#[derive(Deserialize)]
pub struct LinkListQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct StatsQuery {
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequestModel {
    pub old: String,
    pub new: String,
}
