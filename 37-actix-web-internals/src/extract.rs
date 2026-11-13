//! Собственный экстрактор: FromRequest для ClientId.
use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload};

use crate::errors::ApiError;

#[derive(Debug, Clone)]
pub struct ClientId(pub String);

impl FromRequest for ClientId {
    type Error = ApiError;
    // Синхронная проверка заголовка — достаточно Ready, без Box<dyn Future>.
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let res = match req.headers().get("x-client-id") {
            None => Err(ApiError::MissingClientId),
            Some(v) => match v.to_str() {
                Ok(s) if !s.is_empty() && s.len() <= 32 && s.bytes().all(|b| b.is_ascii_alphanumeric()) => {
                    Ok(ClientId(s.to_owned()))
                }
                _ => Err(ApiError::BadClientId(String::from_utf8_lossy(v.as_bytes()).into_owned())),
            },
        };
        ready(res)
    }
}
