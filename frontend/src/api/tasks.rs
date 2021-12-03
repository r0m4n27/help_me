use anyhow::Result;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};

use super::{ApiResult, BearerRequest};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
    pub state: String,
}

pub async fn submit_request(title: &str, description: &str) -> Result<ApiResult<Task>> {
    let payload = json!({
        "title": title,
        "body": description
    });
    let task = Request::post("/api/tasks")
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(task)
}

pub async fn resolve_request(task_id: &str) -> Result<ApiResult<Value>> {
    let response = Request::post(&format!("/api/tasks/{}/resolve", task_id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_task(task_id: &str) -> Result<ApiResult<Task>> {
    let response = Request::get(&format!("/api/tasks/{}", task_id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn update_task(
    task_id: &str,
    title: &str,
    description: &str,
) -> Result<ApiResult<Value>> {
    let payload = json!({
        "title": title,
        "body": description
    });

    let response = Request::patch(&format!("/api/tasks/{}", task_id))
        .body(to_string(&payload).unwrap())
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_tasks(token: &str) -> Result<ApiResult<Vec<Task>>> {
    let response = Request::get("/api/tasks")
        .bearer(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
