use blake2::{Blake2b512, Digest};
use seed::fetch::Result;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{
    user::{get_user, ApiUser},
    ApiResult, BearerRequest,
};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

pub async fn login(user_name: &str, password: &str) -> Result<ApiResult<(Token, ApiUser)>> {
    let payload = json!({
        "user_name": user_name,
        "password": hash_password(password)
    });

    let token_res: ApiResult<Token> = Request::new("/api/auth/login")
        .method(Method::Post)
        .json(&payload)?
        .fetch()
        .await?
        .json()
        .await?;

    match token_res {
        ApiResult::Err(err) => Ok(ApiResult::Err(err)),
        ApiResult::Ok(token) => Ok(get_user(&token.token)
            .await?
            .and_then(move |user| ApiResult::Ok((token, user)))),
    }
}

#[derive(Serialize)]
pub struct RegisterPayload {
    user_name: String,
    hashed_password: String,
    invite_code: Option<String>,
}

impl RegisterPayload {
    pub fn new(user_name: String, password: String, invite_code: String) -> RegisterPayload {
        let invite_code = if invite_code.is_empty() {
            None
        } else {
            Some(invite_code)
        };

        RegisterPayload {
            user_name,
            hashed_password: hash_password(&password),
            invite_code,
        }
    }
}

pub async fn register(payload: &RegisterPayload) -> Result<ApiResult<(Token, ApiUser)>> {
    let token_res: ApiResult<Token> = Request::new("/api/auth/register")
        .method(Method::Post)
        .json(payload)?
        .fetch()
        .await?
        .json()
        .await?;

    match token_res {
        ApiResult::Err(err) => Ok(ApiResult::Err(err)),
        ApiResult::Ok(token) => Ok(get_user(&token.token)
            .await?
            .and_then(move |user| ApiResult::Ok((token, user)))),
    }
}

pub async fn log_out(token: &str) -> Result<ApiResult<Value>> {
    let response = Request::new("/api/auth/logout")
        .method(Method::Post)
        .bearer(token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}

fn hash_password(password: &str) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(password.as_bytes());

    format!("{:x}", hasher.finalize())
}
