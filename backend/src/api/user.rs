use rocket::{serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::models::{Queries, User};

use super::{
    api_result::{ok, ApiError, ApiResult},
    guards::UserGuard,
};

#[derive(Deserialize)]
struct ModifyUserForm {
    user_name: Option<String>,
    password: Option<String>,
}

#[get("/")]
async fn get_user(
    user: Result<UserGuard<'_>, ApiError>,
    queries: &State<Queries>,
) -> ApiResult<User> {
    let db_user = queries.user.get_user(&user?).await?;

    ok(db_user)
}

#[delete("/")]
async fn delete_user(
    user: Result<UserGuard<'_>, ApiError>,
    queries: &State<Queries>,
) -> ApiResult<Value> {
    queries.user.delete_user_token(&user?).await?;
    ok(json!({}))
}

#[patch("/", data = "<data>")]
async fn modify_user(
    user: Result<UserGuard<'_>, ApiError>,
    queries: &State<Queries>,
    data: Json<ModifyUserForm>,
) -> ApiResult<User> {
    let user = user?;

    if let Some(ref user_name) = data.user_name {
        queries.user.update_username(&user, user_name).await?;
    }

    if let Some(ref password) = data.password {
        queries.user.update_password(&user, password).await?;
    }

    let db_user = queries.user.get_user(&user).await?;

    ok(db_user)
}

pub fn user_routes() -> Vec<Route> {
    routes![get_user, delete_user, modify_user]
}
