use serde::{Deserialize, Serialize};

pub mod task;

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
}

#[derive(Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
}
