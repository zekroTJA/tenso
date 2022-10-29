mod middleware;
mod models;
mod routes;
mod tokens;

use crate::{db::DatabaseDriver, util::rand::Rand};
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_service::fn_service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::{Condition, Logger},
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use diesel::row::NamedRow;
use log::warn;
use std::{net, path::Path};

use self::{middleware::xsrf::Xsrf, tokens::TokenHandler};

pub struct Config {
    pub debug_mode: bool,
    pub jwt_signing_key: String,
    pub default_redirect: Option<String>,
    pub notfound_redirect: Option<String>,
    pub origin_url: Option<String>,
    pub asset_dir: String,
}

pub(self) struct State {
    pub initialization_token: Option<String>,
}

impl State {
    fn default() -> Self {
        Self {
            initialization_token: None,
        }
    }
}

pub async fn run<A>(addr: A, cfg: Config, db: DatabaseDriver) -> Result<()>
where
    A: net::ToSocketAddrs,
{
    if cfg.debug_mode {
        warn!("DEBUG MODE IS ENABLED - THIS IS A SECURITY RISK")
    }

    let cfg = Data::new(cfg);
    let db = Data::new(db);
    let token_handler = Data::new(TokenHandler::new(cfg.jwt_signing_key.as_bytes()));
    let mut state = State::default();

    if db.get_users_count()? == 0 {
        let initialization_token = Rand::get(24)?;
        warn!(
            "The app is not initialized - use this token to initialize the app:\n\n{}\n",
            &initialization_token
        );
        state.initialization_token = Some(initialization_token);
    };

    let state = Data::new(state);

    HttpServer::new(move || {
        let asset_dir = cfg.asset_dir.to_owned();

        App::new()
            .app_data(cfg.clone())
            .app_data(db.clone())
            .app_data(token_handler.clone())
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(
                Files::new("/ui", asset_dir)
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let asset_dir = req.app_data::<Data<Config>>().unwrap().asset_dir.clone();
                        let file =
                            NamedFile::open_async(Path::new(&asset_dir).join("index.html")).await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
            .service(
                web::scope("/api")
                    .wrap(Condition::new(
                        cfg.origin_url.is_some(),
                        Cors::default()
                            .allowed_origin(&cfg.origin_url.clone().unwrap_or_default())
                            .allow_any_header()
                            .allow_any_method()
                            .supports_credentials()
                            .max_age(3600),
                    ))
                    .wrap(Condition::new(
                        !cfg.debug_mode,
                        Xsrf::new().cookie_name("xsrf-token").header_name("X-XSRF-Token"),
                    ))
                    .service(web::scope("/auth").configure(routes::auth::register))
                    .service(web::scope("/links").configure(routes::links::register))
                    .service(web::scope("/stats").configure(routes::stats::register)),
            )
            .configure(routes::root::register)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
