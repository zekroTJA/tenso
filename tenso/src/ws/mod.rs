mod auth;
mod models;

use crate::db::DatabaseDriver;
use actix_web::{web::Data, App, HttpServer};
use std::{io, net};

pub async fn run<A>(addr: A, db: DatabaseDriver) -> io::Result<()>
where
    A: net::ToSocketAddrs,
{
    let db = Data::new(db);

    HttpServer::new(move || App::new().app_data(db.clone()).service(auth::check))
        .bind(addr)?
        .run()
        .await
}
