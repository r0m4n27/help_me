use rocket::{Route, State};

use crate::{api::guards::AdminGuard, models::Queries};

use super::{ok, ApiError, ApiResult};
use crate::models::Invite;

#[get("/")]
async fn invites(
    admin: Result<AdminGuard<'_>, ApiError>,
    queries: &State<Queries>,
) -> ApiResult<Vec<Invite>> {
    admin?;
    let invites = queries.invite.get_invites().await?;

    ok(invites)
}

#[delete("/<invite_code>")]
async fn delete_invite(
    admin: Result<AdminGuard<'_>, ApiError>,
    queries: &State<Queries>,
    invite_code: String,
) -> ApiResult<()> {
    admin?;
    queries.invite.delete_invite(&invite_code).await?;

    ok(())
}

#[post("/")]
async fn create_invite(
    admin: Result<AdminGuard<'_>, ApiError>,
    queries: &State<Queries>,
) -> ApiResult<Invite> {
    admin?;
    let invite = queries.invite.create_invite().await?;

    ok(invite)
}

pub fn invite_routes() -> Vec<Route> {
    routes![invites, delete_invite, create_invite]
}
