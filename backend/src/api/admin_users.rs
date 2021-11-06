use rocket::{serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::models::{Queries, User};

use super::{guards::AdminGuard, ok, ApiError, ApiResult};

#[derive(Deserialize)]
struct DeleteUserForm {
    user_name: String,
}

#[get("/users")]
async fn get_users(
    admin: Result<AdminGuard<'_>, ApiError>,
    queries: &State<Queries>,
) -> ApiResult<Vec<User>> {
    admin?;
    let users = queries.user.get_users().await?;

    ok(users)
}

#[delete("/users", data = "<data>")]
async fn delete_user(
    admin: Result<AdminGuard<'_>, ApiError>,
    queries: &State<Queries>,
    data: Json<DeleteUserForm>,
) -> ApiResult<Value> {
    admin?;

    queries.user.delete_user_user_name(&data.user_name).await?;

    ok(json!({}))
}

pub fn admin_routes() -> Vec<Route> {
    routes![get_users, delete_user]
}
