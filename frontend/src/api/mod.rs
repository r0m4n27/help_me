use seed::prelude::*;

use serde::{Deserialize, Serialize};

pub mod admin;
pub mod auth;
pub mod refresh;
pub mod task;
pub mod user;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
// The error should be positioned first because otherwise
// it will always deserialze Value first
pub enum ApiResult<T> {
    Err(ApiError),
    Ok(T),
}

impl<T> ApiResult<T> {
    pub fn map<R, F: FnOnce(T) -> R>(self, func: F) -> ApiResult<R> {
        match self {
            ApiResult::Err(err) => ApiResult::Err(err),
            ApiResult::Ok(data) => ApiResult::Ok(func(data)),
        }
    }

    pub fn and_then<R, F: FnOnce(T) -> ApiResult<R>>(self, func: F) -> ApiResult<R> {
        match self {
            ApiResult::Err(err) => ApiResult::Err(err),
            ApiResult::Ok(data) => func(data),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
}

trait BearerRequest {
    fn bearer(self, token: &str) -> Self;
}

impl BearerRequest for Request<'_> {
    fn bearer(self, token: &str) -> Self {
        self.header(Header::authorization(format!("bearer {}", token)))
    }
}
