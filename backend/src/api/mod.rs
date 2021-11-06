use std::fmt::Debug;

use rocket::{response::Responder, serde::json::Json, Catcher, Route};
use serde::Serialize;
use serde_json::{json, Value};

use crate::models::QueriesError;

use self::{
    admin_users::admin_routes, auth::auth_routes, invite::invite_routes, tasks::tasks_routes,
    user::user_routes,
};

mod admin_users;
mod auth;
mod guards;
mod invite;
mod tasks;
mod user;

#[derive(Debug, Responder)]
pub enum ApiError {
    #[response(status = 500, content_type = "json")]
    Database(Json<ErrorMessage>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorMessage>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorMessage>),
}

impl From<QueriesError> for ApiError {
    fn from(err: QueriesError) -> Self {
        match err {
            QueriesError::Database(_) => ApiError::Database(ErrorMessage::new(
                "Database couldn't handle request!".to_string(),
            )),
            // Thiserror just forwards the internal error
            // so we don't need to call err.to_string()
            QueriesError::ItemNotFound(err) => ApiError::NotFound(ErrorMessage::new(err)),
            QueriesError::IllegalState(err) => ApiError::BadRequest(ErrorMessage::new(err)),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl ErrorMessage {
    fn new(message: String) -> Json<Self> {
        Json(ErrorMessage { message })
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

// A normal function has to be used
// because an impl block can't be used for a type outside of it's crate
fn ok<T, R: From<T>>(data: T) -> ApiResult<R> {
    ApiResult::Ok(Json(R::from(data)))
}

pub fn api_routes() -> Vec<Route> {
    let mut api_routes = Vec::new();

    api_routes.extend(add_base("/auth", auth_routes()));
    api_routes.extend(add_base("/invites", invite_routes()));
    api_routes.extend(add_base("/user", user_routes()));
    api_routes.extend(add_base("/admin", admin_routes()));
    api_routes.extend(add_base("/tasks", tasks_routes()));

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
