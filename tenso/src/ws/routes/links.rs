use crate::{
    db::{models::Link, DatabaseDriver},
    util::links,
    ws::{
        middleware::auth::AuthService,
        models::{LinkCreateRequestModel, LinkListQuery, LinkUpdateRequestModel},
        tokens::Claims,
    },
};
use actix_web::{
    delete, error, get, post,
    web::{self, Data, Json, Path, Query, ServiceConfig},
    Error, HttpResponse,
};
use chrono::Local;

#[get("")]
async fn get_links(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    query: Query<LinkListQuery>,
) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db.list_links(
            &auth.claims().sub,
            query.limit.unwrap_or(50) as i64,
            query.offset.unwrap_or(0) as i64,
            query.search.as_deref(),
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/{id}")]
async fn get_link(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    id: Path<String>,
) -> Result<HttpResponse, Error> {
    _get_link(auth.claims(), id.to_string(), db)
        .await
        .map(|l| HttpResponse::Ok().json(l))
}

#[post("")]
async fn cretae_link(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    link: Json<LinkCreateRequestModel>,
) -> Result<HttpResponse, Error> {
    let _db = db.clone();
    let ident = link.ident.clone();
    let claims = auth.claims();
    let res = web::block(move || _db.get_link(Some(&claims.sub), &ident))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    if res.is_some() {
        return Err(error::ErrorBadRequest("link with ident already exists"));
    }

    let destination = links::normalize(&link.destination).map_err(error::ErrorBadRequest)?;

    let link = Link {
        id: xid::new().to_string(),
        created_date: Local::now().naive_local(),
        creator_id: auth.claims().sub,
        destination,
        enabled: link.enabled,
        ident: link.ident.clone(),
        permanent_redirect: link.permanent_redirect,
    };

    let _db = db.clone();
    let _link = link.clone();
    web::block(move || _db.put_link(&_link))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(&link))
}

#[post("/{id}")]
async fn update_link(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    id: Path<String>,
    link: Json<LinkUpdateRequestModel>,
) -> Result<HttpResponse, Error> {
    let _db = db.clone();
    let res = _get_link(auth.claims(), id.to_string(), _db).await?;

    let _db = db.clone();
    if let Some(ident) = link.ident.clone() {
        let res = web::block(move || _db.get_link(None, &ident))
            .await?
            .map_err(error::ErrorInternalServerError)?;
        if res.is_some() && res.unwrap().id != id.as_str() {
            return Err(error::ErrorBadRequest("link with ident already exists"));
        }
    }

    let destination = links::normalize(link.destination.as_ref().unwrap_or(&res.destination))
        .map_err(error::ErrorBadRequest)?;

    let new_link = Link {
        id: res.id,
        created_date: res.created_date,
        creator_id: res.creator_id,
        ident: link.ident.clone().unwrap_or(res.ident),
        destination,
        enabled: link.enabled.unwrap_or(res.enabled),
        permanent_redirect: link.permanent_redirect.unwrap_or(res.permanent_redirect),
    };

    let _new_link = new_link.clone();
    web::block(move || db.put_link(&_new_link))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(new_link))
}

#[delete("/{id}")]
async fn delete_link(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    id: Path<String>,
) -> Result<HttpResponse, Error> {
    let _db = db.clone();
    _get_link(auth.claims(), id.to_string(), _db).await?;

    web::block(move || db.delete_link(&id))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(get_links)
        .service(get_link)
        .service(cretae_link)
        .service(update_link)
        .service(delete_link);
}

// --- helper ---

async fn _get_link(claims: Claims, id: String, db: Data<DatabaseDriver>) -> Result<Link, Error> {
    let res = web::block(move || db.get_link(Some(&claims.sub), &id))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    match res {
        Some(v) => Ok(v),
        None => Err(error::ErrorNotFound("not found")),
    }
}
