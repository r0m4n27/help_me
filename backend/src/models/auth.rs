use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use sqlx::{Pool, Sqlite};

use crate::models::hash_password;

use super::{generate_random_string, QueriesError, QueriesResult, User, UserType};

// user_name isn't accessed but is needed for the query
#[allow(dead_code)]
#[derive(Debug, FromRow)]
struct UserToken {
    user_name: String,
    token: String,
    expiry: String,
}

impl UserToken {
    // sqlx serialized the datetime
    // so we can also safely deserialize it
    fn expiry(&self) -> DateTime<Utc> {
        DateTime::from_str(&self.expiry).unwrap()
    }
}

pub struct AuthQueries<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> AuthQueries<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        AuthQueries { pool }
    }

    pub async fn create_user(
        &self,
        user_name: &str,
        password: &str,
        user_type: UserType,
    ) -> QueriesResult<()> {
        let password_hash = hash_password(password);
        let user_type_string = user_type.to_string();

        query!(
            "INSERT INTO user(user_name, password_hash, user_type)
            VALUES ($1, $2, $3)",
            user_name,
            password_hash,
            user_type_string
        )
        .execute(self.pool)
        .await?;

        debug!("Created new user: {}", user_name);

        Ok(())
    }

    pub async fn login(&self, user_name: &str, password: &str) -> QueriesResult<String> {
        let user = query_as!(
            User,
            "SELECT * FROM user
            WHERE user_name = ?",
            user_name
        )
        .fetch_optional(self.pool)
        .await?;

        let password_hash = hash_password(password);

        // It would be easier to work with functions like map and and_then
        // but since we're using futures the compiler can't determine the type of an async block
        // and then something like OptionFuture doen't work
        match user {
            Some(user) => {
                if user.password_hash != password_hash {
                    debug!("Wrong password for {} used", user_name);

                    Err(QueriesError::IllegalState(format!(
                        "Password of {} is wrong!",
                        user_name
                    )))
                } else {
                    debug!("User {} logged in", user_name);

                    Ok(self.create_token(user_name).await?)
                }
            }

            None => {
                debug!(
                    "Login for {} requested but user_name is not found!",
                    user_name
                );

                Err(QueriesError::ItemNotFound(format!(
                    "Can't find user {}",
                    user_name
                )))
            }
        }
    }

    pub async fn logout(&self, token: &str) -> QueriesResult<()> {
        query!(
            "DELETE FROM user_token
            WHERE token = ?",
            token
        )
        .execute(self.pool)
        .await?;

        debug!("User with {} logged out", token);

        Ok(())
    }

    pub async fn create_token(&self, user_name: &str) -> QueriesResult<String> {
        let expiry = generate_expiry().to_string();
        let token = generate_random_string(32);

        query!(
            "INSERT INTO user_token(user_name, token, expiry) VALUES ($1, $2, $3)",
            user_name,
            token,
            expiry
        )
        .execute(self.pool)
        .await?;

        debug!("Created token for {}", user_name);

        Ok(token)
    }

    pub async fn is_token_valid(&self, token: &str) -> QueriesResult<bool> {
        let token = query_as!(
            UserToken,
            "SELECT * FROM user_token
            WHERE token = ?",
            token
        )
        .fetch_optional(self.pool)
        .await?;

        match token {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn refresh_token_expiry(&self, token: &str) -> QueriesResult<()> {
        let expiry = generate_expiry().to_string();

        query!(
            "UPDATE user_token
            SET expiry = $1
            WHERE token = $2",
            expiry,
            token
        )
        .execute(self.pool)
        .await?;

        debug!("Refreshed {} token", token);

        Ok(())
    }

    pub async fn is_admin(&self, token: &str) -> QueriesResult<bool> {
        let user = query_as!(
            User,
            "SELECT user.*
            FROM user
            JOIN user_token
            WHERE user.user_name = user_token.user_name
                AND user_token.token = ?
                AND user.user_type = 'admin'",
            token
        )
        .fetch_optional(self.pool)
        .await?;

        match user {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn cleanup_tokens(&self) -> QueriesResult<()> {
        let tokens = query_as!(UserToken, "SELECT * FROM user_token")
            .fetch_all(self.pool)
            .await?;

        let now = Utc::now();
        for token in tokens {
            if token.expiry() < now {
                query!("DELETE FROM user_token WHERE token = ?", token.token)
                    .execute(self.pool)
                    .await?;

                debug!("Cleaned up {} token", token.token);
            }
        }

        Ok(())
    }

    pub async fn invalidate_tokens(&self, user_name: &str) -> QueriesResult<()> {
        query!("DELETE FROM user_token WHERE user_name = ?", user_name)
            .execute(self.pool)
            .await?;

        debug!("Invalidate tokens for {}", user_name);

        Ok(())
    }
}

// After two weeks the user has to login back
fn generate_expiry() -> DateTime<Utc> {
    Utc::now() + Duration::weeks(2)
}
