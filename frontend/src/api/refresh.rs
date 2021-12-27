use seed::fetch::Result;

use super::admin::{get_invites, Invite};
use super::user::{get_users, ApiUser};
use super::ApiResult;

pub async fn refresh_admin(
    token: String,
) -> Result<ApiResult<(String, Vec<Invite>, Vec<ApiUser>)>> {
    let invites_res = get_invites(&token).await?;
    let users_res = get_users(&token).await?;
    let token_res = ApiResult::Ok(token);

    Ok(token_res
        .merge(invites_res)
        .merge(users_res)
        .map(|((token, invites), users)| (token, invites, users)))
}
