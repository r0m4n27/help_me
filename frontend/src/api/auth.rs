use seed::fetch::Result;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    user::{get_user, ApiUser},
    ApiResult,
};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

pub async fn login(user_name: &str, password: &str) -> Result<ApiResult<(Token, ApiUser)>> {
    let payload = json!({
        "user_name": user_name,
        "password": password
    });

    let token_res: ApiResult<Token> = Request::new("/api/auth/login")
        .method(Method::Post)
        .json(&payload)?
        .fetch()
        .await?
        .json()
        .await?;

    match token_res {
        ApiResult::Err(err) => Ok(ApiResult::Err(err)),
        ApiResult::Ok(token) => Ok(get_user(&token.token)
            .await?
            .and_then(move |user| ApiResult::Ok((token, user)))),
    }
}

// #[derive(Serialize)]
// pub struct RegisterPayload<'a> {
//     user_name: &'a str,
//     password: &'a str,
//     invite_code: Option<&'a str>,
// }

// impl<'a> RegisterPayload<'a> {
//     pub fn new(user_name: &'a str, password: &'a str, invite_code: &'a str) -> RegisterPayload<'a> {
//         let invite_code = if invite_code.is_empty() {
//             None
//         } else {
//             Some(invite_code)
//         };

//         RegisterPayload {
//             user_name,
//             password,
//             invite_code,
//         }
//     }
// }

// pub async fn register(payload: &RegisterPayload<'_>) -> Result<ApiResult<Token>> {
//     let token = Request::post("/api/auth/register")
//         .body(to_string(&payload).unwrap())
//         .send()
//         .await?
//         .json()
//         .await?;

//     Ok(token)
// }

// pub async fn log_out(token: &str) -> Result<ApiResult<Value>> {
//     let response = Request::post("/api/auth/logout")
//         .bearer(token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     Ok(response)
// }
