use seed::fetch::Result;
use seed::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{ApiResult, BearerRequest};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ApiUser {
    pub user_name: String,
    pub user_type: String,
}

impl Ord for ApiUser {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.user_name.cmp(&other.user_name)
    }
}

impl PartialOrd for ApiUser {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub async fn get_user(token: &str) -> Result<ApiResult<ApiUser>> {
    let task = Request::new("/api/user")
        .method(Method::Get)
        .bearer(token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(task)
}

pub async fn get_users(token: &str) -> Result<ApiResult<Vec<ApiUser>>> {
    let response = Request::new("/api/admin/users")
        .method(Method::Get)
        .bearer(token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn delete_user(token: &str, user: ApiUser) -> Result<ApiResult<ApiUser>> {
    let payload = json!({ "user_name": &user.user_name });

    let response: ApiResult<Value> = Request::new("/api/admin/users")
        .method(Method::Delete)
        .bearer(token)
        .json(&payload)?
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response.map(|_| user))
}
