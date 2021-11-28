use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ApiResult, BearerRequest};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Invite {
    pub invite_code: String,
}

pub async fn create_invite(token: &str) -> Result<ApiResult<Invite>> {
    let response = Request::post("/api/invites")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn delete_invite(token: &str, invite_code: &str) -> Result<ApiResult<Value>> {
    let response = Request::delete(&format!("/api/invites/{}", invite_code))
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_invites(token: &str) -> Result<ApiResult<Vec<Invite>>> {
    let response = Request::get("/api/invites")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
