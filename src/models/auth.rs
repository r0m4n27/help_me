use std::str::FromStr;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use sqlx::{Pool, Sqlite};

use crate::models::hash_password;

use super::{generate_random_string, User, UserType};

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
    ) -> Result<()> {
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

        Ok(())
    }

    pub async fn login(&self, user_name: &str, password: &str) -> Result<String> {
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
                    Err(anyhow::anyhow!("Password of {} is wrong!", user_name))
                } else {
                    Ok(self.create_token(user_name).await?)
                }
            }

            None => Err(anyhow::anyhow!("Can't find user {}", user_name)),
        }
    }

    pub async fn logout(&self, token: &str) -> Result<()> {
        query!(
            "DELETE FROM user_token
            WHERE token = ?",
            token
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_token(&self, user_name: &str) -> Result<String> {
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

        Ok(token)
    }

    pub async fn is_token_valid(&self, token: &str) -> Result<bool> {
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

    pub async fn refresh_token_expiry(&self, token: &str) -> Result<()> {
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

        Ok(())
    }

    pub async fn is_admin(&self, token: &str) -> Result<bool> {
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

    pub async fn cleanup_tokens(&self) -> Result<()> {
        let tokens = query_as!(UserToken, "SELECT * FROM user_token")
            .fetch_all(self.pool)
            .await?;

        let now = Utc::now();
        for token in tokens {
            if token.expiry() < now {
                query!("DELETE FROM user_token WHERE token = ?", token.token)
                    .execute(self.pool)
                    .await?;
            }
        }

        Ok(())
    }
}

// After two weeks the user has to login back
fn generate_expiry() -> DateTime<Utc> {
    Utc::now() + Duration::weeks(2)
}
