use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

use super::{ApiResult, BearerRequest};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_name: String,
    pub user_type: String,
}

pub async fn get_user(token: &str) -> Result<ApiResult<User>> {
    let task = Request::get("/api/auth/login")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(task)
}
