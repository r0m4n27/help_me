use std::fmt::Debug;

use rocket::{response::Responder, serde::json::Json, Catcher, Route};
use serde::Serialize;
use serde_json::{json, Value};

use self::{auth::auth_routes, invite::invite_routes};

mod auth;
mod guards;
mod invite;

#[derive(Debug, Responder)]
#[response(status = 400, content_type = "json")]
pub struct ApiErrorResponse(Json<ApiError>);

impl From<anyhow::Error> for ApiErrorResponse {
    fn from(err: anyhow::Error) -> Self {
        ApiErrorResponse(Json(ApiError::from(err)))
    }
}

impl ApiErrorResponse {
    pub fn new(message: String) -> Self {
        ApiErrorResponse(Json(ApiError { message }))
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
}

// ThisError should be used instead of anyhow
// otherwise a random user can get internal errors
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError {
            message: err.to_string(),
        }
    }
}

type ApiResult<T> = Result<Json<T>, ApiErrorResponse>;

// A normal function has to be used
// because an impl block can't be used for a type outside of it's crate
fn ok<T>(data: T) -> ApiResult<T> {
    ApiResult::Ok(Json(data))
}

pub fn api_routes() -> Vec<Route> {
    let mut api_routes = Vec::new();

    api_routes.extend(add_base("/auth", auth_routes()));
    api_routes.extend(add_base("/invites", invite_routes()));

    api_routes
}

#[catch(404)]
async fn not_found() -> Value {
    json!({"message": "Resource was not found!"})
}

pub fn api_catchers() -> Vec<Catcher> {
    catchers![not_found]
}

fn add_base(base: &str, routes: Vec<Route>) -> Vec<Route> {
    routes
        .into_iter()
        .map(|route| {
            route
                .map_base(|origin| format!("{}{}", base, origin))
                .expect("Route coudn't be created!")
        })
        .collect()
}
