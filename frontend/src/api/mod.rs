use serde::{Deserialize, Serialize};

pub mod tasks;

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
