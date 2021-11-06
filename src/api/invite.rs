use anyhow::Result;
use rocket::{Route, State};

use crate::{api::guards::Admin, models::Queries};

use super::{ok, ApiResult};
use crate::models::Invite;

#[get("/")]
async fn invites(admin: Result<Admin<'_>>, queries: &State<Queries>) -> ApiResult<Vec<Invite>> {
    admin?;
    let invites = queries.invite.get_invites().await?;

    ok(invites)
}

#[delete("/<invite_code>")]
async fn delete_invite(
    admin: Result<Admin<'_>>,
    queries: &State<Queries>,
    invite_code: String,
) -> ApiResult<()> {
    admin?;
    queries.invite.delete_invite(&invite_code).await?;

    ok(())
}

#[post("/")]
async fn create_invite(admin: Result<Admin<'_>>, queries: &State<Queries>) -> ApiResult<Invite> {
    admin?;
    let invite = queries.invite.create_invite().await?;

    ok(invite)
}

pub fn invite_routes() -> Vec<Route> {
    routes![invites, delete_invite, create_invite]
}
