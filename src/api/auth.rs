use anyhow::Result;
use rocket::{serde::json::Json, Route, State};
use serde_json::{json, Value};

use super::{ok, ApiErrorResponse, ApiResult};
use crate::{
    api::guards::User,
    models::{Queries, UserType},
};

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct LoginData {
    user_name: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterData {
    user_name: String,
    password: String,
    invite_code: Option<String>,
}

#[post("/register", data = "<data>")]
async fn register(data: Json<RegisterData>, queries: &State<Queries>) -> ApiResult<LoginResponse> {
    let user_type = if data.invite_code.is_some() {
        UserType::Tutor
    } else {
        UserType::Admin
    };

    if let Some(invite_code) = data.invite_code.as_ref() {
        if queries.invite.invite_exists(invite_code).await? {
            queries.invite.delete_invite(invite_code).await?;
        } else {
            return Err(ApiErrorResponse::new(
                "Wrong invite_code provided!".to_string(),
            ));
        }
    }

    queries
        .auth
        .create_user(&data.user_name, &data.password, user_type)
        .await?;

    let token = queries.auth.create_token(&data.user_name).await?;

    ok(LoginResponse { token })
}

#[post("/login", data = "<data>")]
async fn login(data: Json<LoginData>, queries: &State<Queries>) -> ApiResult<LoginResponse> {
    let token = queries.auth.login(&data.user_name, &data.password).await?;

    ok(LoginResponse { token })
}

// It is not possible to use catchers to catch failures that happen in FromRequest
// but we can try to get an Result and use it instead
#[post("/logout")]
async fn logout(token: Result<User<'_>>, queries: &State<Queries>) -> ApiResult<Value> {
    queries.auth.logout(&token?).await?;

    ok(json!({}))
}

pub fn auth_routes() -> Vec<Route> {
    routes![register, login, logout]
}
