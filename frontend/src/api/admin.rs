use seed::fetch::Result;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ApiResult, BearerRequest};

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Invite {
    pub invite_code: String,
}

pub async fn create_invite(token: &str) -> Result<ApiResult<Invite>> {
    let response = Request::new("/api/invites")
        .method(Method::Post)
        .bearer(token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn delete_invite(token: &str, invite: Invite) -> Result<ApiResult<Invite>> {
    let response: ApiResult<Value> = Request::new(&format!("/api/invites/{}", invite.invite_code))
        .method(Method::Delete)
        .bearer(token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response.map(|_| invite))
}

pub async fn get_invites(token: &str) -> Result<ApiResult<Vec<Invite>>> {
    let response = Request::new("/api/invites")
        .method(Method::Get)
        .bearer(token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}
