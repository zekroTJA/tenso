use crate::ws::tokens::{Claims, TokenHandler};
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use log::debug;

pub struct AuthService {
    claims: Claims,
}

impl AuthService {
    pub fn claims(&self) -> Claims {
        self.claims.clone()
    }
}

impl FromRequest for AuthService {
    type Error = Error;
    type Future = Ready<Result<AuthService, Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let token_handler = req
            .app_data::<Data<TokenHandler>>()
            .unwrap();

        let cookie = req.cookie("token");
        if cookie.is_none() {
            debug!("no token cookie");
            return err(ErrorUnauthorized("unauthorized"));
        }

        let token_result = token_handler.decode::<Claims>(cookie.unwrap().value());

        match token_result {
            Ok(claims) => ok(AuthService { claims }),
            Err(e) => {
                debug!("token decode error: {}", e);
                err(ErrorUnauthorized("unauthorized"))
            }
        }
    }
}
