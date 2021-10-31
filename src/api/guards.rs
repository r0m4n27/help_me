use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use std::ops::Deref;

use crate::models::Queries;

pub struct Token<'r>(&'r str);

impl<'r> Deref for Token<'r> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// TODO: Bad error handling
#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
    Db(anyhow::Error),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Token<'r>, Self::Error> {
        // There are always queries available
        let queries = req.rocket().state::<Queries>().unwrap();

        let token = req.headers().get_one("Authorization");

        // The FromResidual trait is a nightly feature
        // otherwise the '?' operator could be used
        match token {
            Some(token) => match queries.auth.is_token_valid(token).await {
                Ok(valid) => {
                    if valid {
                        if let Err(err) = queries.auth.refresh_token_expiry(token).await {
                            return Outcome::Failure((Status::BadRequest, TokenError::Db(err)));
                        }

                        Outcome::Success(Token(token))
                    } else {
                        Outcome::Failure((Status::BadRequest, TokenError::Invalid))
                    }
                }
                Err(err) => Outcome::Failure((Status::BadRequest, TokenError::Db(err))),
            },
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
        }
    }
}
