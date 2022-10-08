use actix_web::{
    error, get,
    web::{self, Data},
    Error, HttpResponse,
};

use crate::{db::DatabaseDriver, ws::models::*};

#[get("/test")]
pub async fn check(db: Data<DatabaseDriver>) -> Result<HttpResponse, Error> {
    let res = web::block(move || db.get_auth_user("root")).await?;
    let res = res.map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(AuthCheckResponseModel {
        initialized: res.is_some(),
    }))
}
