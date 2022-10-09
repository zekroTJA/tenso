use actix_web::{
    get,
    web::{Data, ServiceConfig},
    Error, HttpResponse,
};

use crate::{db::DatabaseDriver, ws::middleware::auth::AuthService};

#[get("")]
async fn get_links(auth: AuthService, db: Data<DatabaseDriver>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(get_links);
}
