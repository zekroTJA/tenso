use crate::{
    db::{models::StatEntry, DatabaseDriver},
    ws::Config,
};
use actix_web::{
    error, get,
    web::{self, Data, Path, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};

#[get("/")]
async fn get_root(cfg: Data<Config>) -> Result<HttpResponse, Error> {
    if let Some(default_redirect) = &cfg.default_redirect {
        Ok(HttpResponse::Found()
            .append_header(("Location", default_redirect.to_string()))
            .finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/{ident}")]
async fn get_redirect(
    db: Data<DatabaseDriver>,
    cfg: Data<Config>,
    ident: Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let _db = db.clone();
    let res = web::block(move || _db.get_link_by_ident(&ident))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    if res.is_none() || !res.clone().unwrap().enabled {
        return if let Some(notfound_redirect) = &cfg.notfound_redirect {
            Ok(HttpResponse::TemporaryRedirect()
                .append_header(("Location", notfound_redirect.to_string()))
                .finish())
        } else {
            Err(error::ErrorNotFound("not found"))
        };
    }

    let link = res.unwrap();
    let mut entry = StatEntry::from(&link);
    entry.user_agent = req
        .headers()
        .get("User-Agent")
        .map(|v| v.to_str().unwrap_or_default().to_string());

    tokio::spawn(web::block(move || {
        let res = db.put_stats(&entry);
        if let Err(err) = res {
            log::error!("Failed storing redirect stats to database: {err}");
        }
    }));

    let mut response = if link.permanent_redirect {
        HttpResponse::PermanentRedirect()
    } else {
        HttpResponse::TemporaryRedirect()
    };
    Ok(response.append_header(("Location", link.destination)).finish())
}

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(get_root);
    cfg.service(get_redirect);
}
