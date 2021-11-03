use rocket::{Route, State};

use crate::{api::guards::Admin, models::Queries};

use super::{ok, ApiResult};
use crate::models::invite::Invite;

#[get("/")]
async fn invites(_admin: Admin<'_>, queries: &State<Queries>) -> ApiResult<Vec<Invite>> {
    let invites = queries.invite.get_invites().await?;

    ok(invites)
}

#[delete("/<invite_code>")]
async fn delete_invite(
    _admin: Admin<'_>,
    queries: &State<Queries>,
    invite_code: String,
) -> ApiResult<()> {
    queries.invite.delete_invite(&invite_code).await?;

    ok(())
}

#[post("/")]
async fn create_invite(_admin: Admin<'_>, queries: &State<Queries>) -> ApiResult<Invite> {
    let invite = queries.invite.create_invite().await?;

    ok(invite)
}

pub fn invite_routes() -> Vec<Route> {
    routes![invites, delete_invite, create_invite]
}
