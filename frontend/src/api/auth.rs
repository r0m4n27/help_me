use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};

use super::{ApiResult, BearerRequest};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

pub async fn login(user_name: &str, password: &str) -> Result<ApiResult<Token>> {
    let payload = json!({
        "user_name": user_name,
        "password": password
    });
    let token = Request::post("/api/auth/login")
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(token)
}
#[derive(Serialize)]
pub struct RegisterPayload<'a> {
    user_name: &'a str,
    password: &'a str,
    invite_code: Option<&'a str>,
}

impl<'a> RegisterPayload<'a> {
    pub fn new(user_name: &'a str, password: &'a str, invite_code: &'a str) -> RegisterPayload<'a> {
        let invite_code = if invite_code.is_empty() {
            None
        } else {
            Some(invite_code)
        };

        RegisterPayload {
            user_name,
            password,
            invite_code,
        }
    }
}

pub async fn register(payload: &RegisterPayload<'_>) -> Result<ApiResult<Token>> {
    let token = Request::post("/api/auth/register")
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(token)
}

pub async fn log_out(token: &str) -> Result<ApiResult<Value>> {
    let response = Request::post("/api/auth/logout")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
