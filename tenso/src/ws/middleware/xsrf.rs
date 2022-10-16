use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_service::Transform;
use actix_web::{
    cookie::{Cookie, Expiration, SameSite},
    dev::{Service, ServiceRequest, ServiceResponse},
    error, Error,
};
use futures::{future::LocalBoxFuture, FutureExt};

use crate::util::rand::Rand;

pub struct XsrfMiddleware<S> {
    service: Rc<S>,
    cookie_name: String,
    header_name: String,
}

impl<S, B> Service<ServiceRequest> for XsrfMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        let token_cookie = req.cookie(&self.cookie_name);

        if req.method() == "GET" && token_cookie.is_none() {
            let name = self.cookie_name.clone();

            return async move {
                let token = Self::get_token().map_err(error::ErrorInternalServerError)?;
                let mut res = srv.call(req).await?;
                res.response_mut().add_cookie(
                    &Cookie::build(name, token)
                        .path("/")
                        .secure(true)
                        .http_only(false)
                        .same_site(SameSite::Strict)
                        .expires(Expiration::Session)
                        .finish(),
                )?;
                Ok(res)
            }
            .boxed_local();
        }

        if req.method() != "GET" && req.method() != "OPTIONS" {
            let token_header = req.headers().get(&self.header_name);
            if token_header.is_none()
                || token_cookie.is_none()
                || token_header.unwrap().to_str().unwrap_or("") != token_cookie.unwrap().value()
            {
                return ready(Err(error::ErrorForbidden("invalid xsrf token"))).boxed_local();
            }
        }

        srv.call(req).boxed_local()
    }
}

impl<S> XsrfMiddleware<S> {
    #[inline]
    fn get_token() -> Result<String, getrandom::Error> {
        Rand::get(16)
    }
}

pub struct Xsrf<'a> {
    cookie_name: Option<&'a str>,
    header_name: Option<&'a str>,
}

impl<'a> Xsrf<'a> {
    pub fn new() -> Self {
        Self {
            cookie_name: None,
            header_name: None,
        }
    }

    pub fn cookie_name(mut self, name: &'a str) -> Self {
        self.cookie_name = Some(name);
        self
    }

    pub fn header_name(mut self, header: &'a str) -> Self {
        self.header_name = Some(header);
        self
    }
}

impl<'a, S, B> Transform<S, ServiceRequest> for Xsrf<'a>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = XsrfMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(XsrfMiddleware {
            service: Rc::new(service),
            cookie_name: self.cookie_name.unwrap_or("xsrf-token").to_string(),
            header_name: self.header_name.unwrap_or("X-XSRF-Token").to_string(),
        }))
    }
}
