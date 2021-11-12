use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiError {
    pub message: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Debug)]
pub struct CreateTaskPayload {
    title: String,
    body: String,
}

impl CreateTaskPayload {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            body: description,
        }
    }
}

pub async fn submit_request(payload: CreateTaskPayload) -> Result<ApiResult<Task>> {
    let task = Request::post("/api/tasks")
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(task)
}
