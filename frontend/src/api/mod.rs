use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod tasks;
pub mod user;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
// The error should be positioned first because otherwise
// it will always deserialze Value first
pub enum ApiResult<T> {
    Err(ApiError),
    Ok(T),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ApiError {
    pub message: String,
}

trait BearerRequest {
    fn bearer(self, token: &str) -> Self;
}

impl BearerRequest for Request {
    fn bearer(self, token: &str) -> Self {
        self.header("Authorization", &format!("bearer {}", token))
    }
}
