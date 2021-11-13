use serde::{Deserialize, Serialize};

pub mod tasks;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ApiError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct NoResult;
