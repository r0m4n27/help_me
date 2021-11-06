use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::models::hash_password;

use super::User;

pub struct UserQueries<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> UserQueries<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        UserQueries { pool }
    }

    async fn get_user_optional(&self, token: &str) -> Result<Option<User>> {
        let user = query_as!(
            User,
            "SELECT user.*
            FROM user
            JOIN user_token
            WHERE user.user_name = user_token.user_name
                AND user_token.token = ?",
            token
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user(&self, token: &str) -> Result<User> {
        self.get_user_optional(token)
            .await?
            .ok_or_else(|| anyhow!("User for token {} not found!", token))
    }

    pub async fn delete_user(&self, token: &str) -> Result<()> {
        let user = self.get_user_result(token).await?;

        query!("DELETE FROM user WHERE user_name = ?", user.user_name)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_username(&self, token: &str, new_username: &str) -> Result<()> {
        let user = self.get_user_result(token).await?;

        query!(
            "UPDATE user
            SET user_name = $1
            WHERE user_name = $2",
            new_username,
            user.user_name
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_password(&self, token: &str, new_password: &str) -> Result<()> {
        let user = self.get_user_result(token).await?;

        let hashed_password = hash_password(new_password);
        query!(
            "UPDATE user
            SET password_hash = $1
            WHERE user_name = $2",
            new_password,
            user.user_name
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }
}
