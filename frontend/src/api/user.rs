use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};

use super::{ApiResult, BearerRequest};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub user_name: String,
    pub user_type: String,
}

pub async fn get_user(token: &str) -> Result<ApiResult<User>> {
    let task = Request::get("/api/user")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(task)
}

pub async fn get_users_admin(token: &str) -> Result<ApiResult<Vec<User>>> {
    let response = Request::get("/api/admin/users")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn delete_user_admin(token: &str, user_name: &str) -> Result<ApiResult<Value>> {
    let payload = json!({ "user_name": user_name });

    let response = Request::delete("/api/admin/users")
        .bearer(token)
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
