use serde::Serialize;

#[derive(Serialize)]
pub struct AuthCheckResponseModel {
    pub initialized: bool,
}
