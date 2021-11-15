use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};

use super::ApiResult;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

pub async fn login(user_name: &str, password: &str) -> Result<ApiResult<Token>> {
    let payload = json!({
        "user_name": user_name,
        "password": password
    });
    let task = Request::post("/api/auth/login")
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(task)
}
