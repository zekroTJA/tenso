mod middleware;
mod models;
mod routes;
mod tokens;

use crate::db::DatabaseDriver;
use actix_cors::Cors;
use actix_web::{
    middleware::{Condition, Logger},
    web::{self, Data},
    App, HttpServer,
};
use log::warn;
use std::{io, net};

use self::tokens::TokenHandler;

pub struct Config {
    pub debug_mode: bool,
    pub jwt_signing_key: String,
    pub default_redirect: Option<String>,
    pub notfound_redirect: Option<String>,
    pub origin_url: Option<String>,
}

pub async fn run<A>(addr: A, cfg: Config, db: DatabaseDriver) -> io::Result<()>
where
    A: net::ToSocketAddrs,
{
    if cfg.debug_mode {
        warn!("DEBUG MODE IS ENABLED - THIS IS A SECURITY RISK")
    }

    let cfg = Data::new(cfg);
    let db = Data::new(db);
    let token_handler = Data::new(TokenHandler::new(cfg.jwt_signing_key.as_bytes()));

    HttpServer::new(move || {
        App::new()
            .app_data(cfg.clone())
            .app_data(db.clone())
            .app_data(token_handler.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .wrap(Condition::new(
                        cfg.origin_url.is_some(),
                        Cors::default()
                            .allowed_origin(
                                &cfg.origin_url
                                    .clone()
                                    .unwrap_or_default(),
                            )
                            .allow_any_header()
                            .allow_any_method()
                            .supports_credentials()
                            .max_age(3600),
                    ))
                    .service(web::scope("/auth").configure(routes::auth::register))
                    .service(web::scope("/links").configure(routes::links::register))
                    .service(web::scope("/stats").configure(routes::stats::register)),
            )
            .configure(routes::root::register)
    })
    .bind(addr)?
    .run()
    .await
}
