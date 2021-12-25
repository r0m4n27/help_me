use seed::fetch::Result;
use seed::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::msg::api::ResponseApiMsg;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
    pub state: String,
}

pub async fn submit_task(title: &str, description: &str) -> Result<ResponseApiMsg> {
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

    Ok(ResponseApiMsg::Submit(response))
}

pub async fn resolve_task(task_id: &str) -> Result<ResponseApiMsg> {
    let response = Request::new(&format!("/api/tasks/{}/resolve", task_id))
        .method(Method::Post)
        .fetch()
        .await?
        .json()
        .await?;

    Ok(ResponseApiMsg::Resolve(response))
}

pub async fn update_task(task_id: &str, title: &str, description: &str) -> Result<ResponseApiMsg> {
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

    Ok(ResponseApiMsg::Edit(response))
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

// pub async fn get_task(task_id: &str) -> Result<ApiResult<Task>> {
//     let response = Request::get(&format!("/api/tasks/{}", task_id))
//         .send()
//         .await?
//         .json()
//         .await?;

//     Ok(response)
// }

// pub async fn get_tasks(token: &str) -> Result<ApiResult<Vec<Task>>> {
//     let response = Request::get("/api/tasks")
//         .bearer(token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     Ok(response)
// }
