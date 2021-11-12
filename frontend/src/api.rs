use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "http://127.0.0.1:8000/api";

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
    id: String,
    title: String,
    body: String,
}

#[derive(Serialize)]
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

pub async fn submit_request(payload: CreateTaskPayload) -> Task {
    Client::new()
        .post(format!("{}/tasks", API_URL))
        .json(&payload)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
