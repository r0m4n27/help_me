use seed::fetch::Result;

use super::admin::{get_invites, Invite};
use super::ApiResult;

pub async fn refresh_admin(token: String) -> Result<ApiResult<(String, Vec<Invite>)>> {
    let response = get_invites(&token).await?.map(|invites| (token, invites));

    Ok(response)
}
