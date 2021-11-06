use rocket::{serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::{
    api::ok,
    models::{self, Queries},
};

use super::{guards::User, ApiResult};

#[derive(Serialize)]
struct UserResponse {
    user_name: String,
    user_type: String,
}

impl From<models::User> for UserResponse {
    fn from(user: models::User) -> Self {
        UserResponse {
            user_name: user.user_name,
            user_type: user.user_type,
        }
    }
}

#[derive(Deserialize)]
struct ModifyUserForm {
    user_name: Option<String>,
    password: Option<String>,
}

#[get("/")]
async fn get_user(user: User<'_>, queries: &State<Queries>) -> ApiResult<UserResponse> {
    let db_user = queries.user.get_user(&user).await?;

    ok(db_user)
}

#[delete("/")]
async fn delete_user(user: User<'_>, queries: &State<Queries>) -> ApiResult<Value> {
    queries.user.delete_user(&user).await?;
    ok(json!({}))
}

#[patch("/", data = "<data>")]
async fn modify_user(
    user: User<'_>,
    queries: &State<Queries>,
    data: Json<ModifyUserForm>,
) -> ApiResult<UserResponse> {
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
