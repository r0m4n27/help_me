use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome},
    request::{FromRequest, Outcome},
    Request,
};
use std::ops::Deref;

use crate::models::Queries;

pub struct User<'r>(pub &'r str);

impl<'r> Deref for User<'r> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User<'r> {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<User<'r>, Self::Error> {
        // We have to use try_outcome because FromResidual for '?' is a nightly feature
        // The compiler somehow thinks this would be '!' rather than '&Queries'
        let queries: &Queries = try_outcome!(req.rocket().state::<Queries>().or_forward(()));
        let token = try_outcome!(req
            .headers()
            .get_one("Authorization")
            .and_then(|text| extract_bearer_token(text))
            .or_forward(()));

        let token_valid = try_outcome!(queries
            .auth
            .is_token_valid(token)
            .await
            .into_outcome(Status::BadRequest));

        if token_valid {
            queries
                .auth
                .refresh_token_expiry(token)
                .await
                .into_outcome(Status::BadRequest)
                .map(|_| User(token))
        } else {
            Outcome::Failure((Status::BadRequest, anyhow!("Provided token is invalid!")))
        }
    }
}

pub struct Admin<'r>(pub &'r str);

impl<'r> Deref for Admin<'r> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin<'r> {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Admin<'r>, Self::Error> {
        // We have to use try_outcome because FromResidual for '?' is a nightly feature
        // The compiler somehow thinks this would be '!' rather than '&Queries'
        let queries: &Queries = try_outcome!(req.rocket().state::<Queries>().or_forward(()));
        let user: User = try_outcome!(req.guard::<User<'r>>().await);

        let is_admin = try_outcome!(queries
            .auth
            .is_admin(user.0)
            .await
            .into_outcome(Status::BadRequest));

        if is_admin {
            Outcome::Success(Admin(user.0))
        } else {
            Outcome::Failure((
                Status::BadRequest,
                anyhow!("Authenticated user is not an admin!"),
            ))
        }
    }
}

fn extract_bearer_token(text: &str) -> Option<&str> {
    let parts = text.split_whitespace().collect::<Vec<_>>();
    println!("{:?}", parts);

    if parts.len() == 2 {
        if parts[0].to_lowercase() == "bearer" {
            Some(parts[1])
        } else {
            None
        }
    } else {
        None
    }
}
