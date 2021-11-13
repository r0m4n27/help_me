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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ApiError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct NoResult;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
    pub state: String,
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

pub async fn submit_request(payload: &CreateTaskPayload) -> Result<ApiResult<Task>> {
    let task = Request::post("/api/tasks")
        .body(to_string(payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(task)
}

pub async fn resolve_request(task_id: &String) -> Result<ApiResult<NoResult>> {
    let response = Request::post(&format!("/api/tasks/{}/resolve", task_id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_task(task_id: &String) -> Result<ApiResult<Task>> {
    let response = Request::get(&format!("/api/tasks/{}", task_id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
