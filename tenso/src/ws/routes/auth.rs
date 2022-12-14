use crate::{
    db::{models::AuthUser, DatabaseDriver},
    util::rand::Rand,
    ws::{
        middleware::auth::AuthService,
        models::*,
        tokens::{Claims, TokenHandler},
        Config, State,
    },
};
use actix_web::{
    cookie::{time::Duration, Cookie},
    error, get, post,
    web::{self, Data, Json, ServiceConfig},
    Error, HttpResponse,
};
use log::debug;

const TOKEN_LIFETIME_SECONDS: usize = 3600 * 24 * 30; // 30 days

#[get("/init")]
async fn get_init(db: Data<DatabaseDriver>) -> Result<HttpResponse, Error> {
    let res = web::block(move || db.get_auth_user("root")).await?;
    let res = res.map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(AuthCheckResponseModel {
        initialized: res.is_some(),
    }))
}

#[post("/init")]
async fn post_init(
    db: Data<DatabaseDriver>,
    state: Data<State>,
    p: Json<AuthInitRequestModel>,
) -> Result<HttpResponse, Error> {
    if p.username.is_empty() {
        return Err(error::ErrorBadRequest("username must not be empty"));
    }
    if p.password.is_empty() {
        return Err(error::ErrorBadRequest("password must not be empty"));
    }

    let _db = db.clone();
    let res = web::block(move || _db.get_users_count())
        .await?
        .map_err(error::ErrorInternalServerError)?;
    if res > 0 {
        return Err(error::ErrorBadRequest("already initialized"));
    }

    if state.initialization_token.is_none() {
        return Err(error::ErrorInternalServerError("initialization token not generated; this should actually not happen, please restart the app and try again"));
    }

    if let Some(token) = &state.initialization_token {
        if token != &p.token {
            return Err(error::ErrorBadRequest("invalid initialization token"));
        }
    } else {
        return Err(error::ErrorInternalServerError("initialization token not generated; this should actually not happen, please restart the app and try again"));
    }

    let password_hash = hash_password(&p.password)?;

    let root_user = AuthUser {
        username: (&p.username).into(),
        password_hash,
    };

    let _db = db.clone();
    web::block(move || _db.put_auth_user(&root_user))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/login")]
async fn post_login(
    cfg: Data<Config>,
    db: Data<DatabaseDriver>,
    token_handler: Data<TokenHandler>,
    p: Json<AuthLoginRequestModel>,
) -> Result<HttpResponse, Error> {
    debug!("payload: {:#?}", p);

    if p.username.is_empty() || p.password.is_empty() {
        return Err(error::ErrorUnauthorized("unauthorized"));
    }

    let username = p.username.clone();
    let res = web::block(move || db.get_auth_user(&username))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    if res.is_none() {
        return Err(error::ErrorUnauthorized("unauthorized"));
    }

    check_password_hash(&res.unwrap().password_hash, &p.password)?;

    let token = token_handler
        .encode(&Claims::new(&p.username, TOKEN_LIFETIME_SECONDS))
        .map_err(error::ErrorInternalServerError)?;
    let cookie = Cookie::build("token", token)
        .path("/")
        .secure(!cfg.debug_mode)
        .max_age(Duration::seconds(TOKEN_LIFETIME_SECONDS as i64))
        .http_only(true)
        .finish();

    Ok(HttpResponse::NoContent().cookie(cookie).finish())
}

#[get("/check")]
async fn get_check(auth: AuthService) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(auth.claims()))
}

#[post("/password")]
async fn post_password(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    pw: Json<ChangePasswordRequestModel>,
) -> Result<HttpResponse, Error> {
    if pw.old.is_empty() {
        return Err(error::ErrorBadRequest("old password can not be empty"));
    }
    if pw.new.is_empty() {
        return Err(error::ErrorBadRequest("new password can not be empty"));
    }

    let _db = db.clone();
    let user = web::block(move || _db.get_auth_user(&auth.claims().sub))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    if user.is_none() {
        return Err(error::ErrorUnauthorized("unauthorized"));
    }

    let mut user = user.unwrap();
    check_password_hash(&user.password_hash, &pw.old)
        .map_err(|_| error::ErrorBadRequest("old password is wrong"))?;

    user.password_hash = hash_password(&pw.new)?;

    web::block(move || db.put_auth_user(&user))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(get_init);
    cfg.service(post_init);
    cfg.service(post_login);
    cfg.service(get_check);
    cfg.service(post_password);
}

// -- helpers --

#[inline]
fn check_password_hash(hash: &str, password: &str) -> Result<(), Error> {
    let matches = argon2::verify_encoded(hash, password.as_bytes())
        .map_err(error::ErrorInternalServerError)?;
    if !matches {
        Err(error::ErrorUnauthorized("unauthorized"))
    } else {
        Ok(())
    }
}

fn hash_password(password: &str) -> Result<String, Error> {
    let cfg = argon2::Config::default();
    let salt: Vec<u8> = Rand::get(32).map_err(error::ErrorInternalServerError)?;
    let password_hash = argon2::hash_encoded(password.as_bytes(), &salt, &cfg)
        .map_err(error::ErrorInternalServerError)?;
    Ok(password_hash)
}
