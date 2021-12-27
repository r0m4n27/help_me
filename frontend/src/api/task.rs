use std::str::FromStr;

use chrono::{DateTime, Utc};
use seed::fetch::Result;
use seed::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{ApiResult, BearerRequest};

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
    pub state: String,
    created_at: String,
}

impl Task {
    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_str(&self.created_at).unwrap()
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.created_at().cmp(&other.created_at())
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.created_at().cmp(&other.created_at()))
    }
}

pub async fn submit_task(title: &str, description: &str) -> Result<ApiResult<Task>> {
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

pub async fn resolve_task(task_id: &str) -> Result<ApiResult<Task>> {
    let response = Request::new(&format!("/api/tasks/{}/resolve", task_id))
        .method(Method::Post)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn update_task(task_id: &str, title: &str, description: &str) -> Result<ApiResult<Task>> {
    let payload = json!({
        "title": title,
        "body": description
    });

    let response = Request::new(&format!("/api/tasks/{}", task_id))
        .method(Method::Patch)
        .json(&payload)?
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_task(task_id: &str) -> Result<ApiResult<Task>> {
    let response = Request::new(&format!("/api/tasks/{}", task_id))
        .method(Method::Get)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_tasks(token: String) -> Result<ApiResult<(String, Vec<Task>)>> {
    let response: ApiResult<Vec<Task>> = Request::new("/api/tasks")
        .method(Method::Get)
        .bearer(&token)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(response.map(|tasks| (token, tasks)))
}

// pub async fn process_task(token: &str, task_id: &str) -> Result<ApiResult<Value>> {
//     let response = Request::post(&format!("/api/tasks/{}/start", task_id))
//         .bearer(token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     Ok(response)
// }

// pub async fn finish_task(token: &str, task_id: &str) -> Result<ApiResult<Value>> {
//     let response = Request::post(&format!("/api/tasks/{}/complete", task_id))
//         .bearer(token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     Ok(response)
// }
