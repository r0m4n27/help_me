use seed::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::ApiResult;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
    pub state: String,
}

pub async fn submit_task(title: &str, description: &str) -> fetch::Result<ApiResult<Task>> {
    let payload = json!({
        "title": title,
        "body": description
    });

    let response = Request::new("/api/tasks")
        .method(Method::Post)
        .json(&payload)?
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}
