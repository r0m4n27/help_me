use rocket::{serde::json::Json, Route, State};
use serde_json::{json, Value};

use super::{guards::Token, ApiResult};
use crate::models::{Queries, UserType};

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct UserAuth {
    user_name: String,
    password: String,
}

#[post("/register_admin", data = "<auth>")]
async fn register_admin(
    auth: Json<UserAuth>,
    queries: &State<Queries>,
) -> ApiResult<LoginResponse> {
    queries
        .auth
        .create_user(&auth.user_name, &auth.password, UserType::Admin)
        .await?;

    let token = queries.auth.create_token(&auth.user_name).await?;

    ApiResult::Ok(Json(LoginResponse { token }))
}

#[post("/login", data = "<auth>")]
async fn login(auth: Json<UserAuth>, queries: &State<Queries>) -> ApiResult<LoginResponse> {
    let token = queries.auth.login(&auth.user_name, &auth.password).await?;

    ApiResult::Ok(Json(LoginResponse { token }))
}

#[post("/logout")]
async fn logout(token: Token<'_>, queries: &State<Queries>) -> ApiResult<Value> {
    queries.auth.logout(&token).await?;

    ApiResult::Ok(Json(json!({})))
}

pub fn auth_routes() -> Vec<Route> {
    routes![register_admin, login, logout]
}
