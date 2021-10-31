use rocket::{response::Responder, serde::json::Json, Catcher, Route};
use serde::Serialize;
use serde_json::{json, Value};

use self::auth::auth_routes;

mod auth;
mod guards;

#[derive(Serialize, Responder)]
#[response(status = 400, content_type = "json")]
struct ApiError {
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

type ApiResult<T> = Result<Json<T>, ApiError>;

pub fn api_routes() -> Vec<Route> {
    let mut api_routes = Vec::new();

    api_routes.extend(add_base("/auth", auth_routes()));

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
