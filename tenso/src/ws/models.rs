use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthCheckResponseModel {
    pub initialized: bool,
}

#[derive(Deserialize)]
pub struct AuthInitRequestModel {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthLoginRequestModel {
    pub username: String,
    pub password: String,
}
