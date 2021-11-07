use sqlx::{query, query_as, Pool, Sqlite};

use crate::models::{hash_password, QueriesError};

use super::{QueriesResult, User};

pub struct UserQueries<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> UserQueries<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        UserQueries { pool }
    }

    pub async fn get_user(&self, token: &str) -> QueriesResult<User> {
        query_as!(
            User,
            "SELECT user.*
            FROM user
            JOIN user_token
            WHERE user.user_name = user_token.user_name
                AND user_token.token = ?",
            token
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|err| err.into())
        .and_then(|user| {
            user.map(|user| {
                debug!("Requested user {}", user.user_name);
                user
            })
            .ok_or_else(|| {
                QueriesError::ItemNotFound(format!("User for token {} not found!", token))
            })
        })
    }

    pub async fn get_users(&self) -> QueriesResult<Vec<User>> {
        query_as!(
            User,
            "SELECT *
            FROM user"
        )
        .fetch_all(self.pool)
        .await
        .map(|users| {
            debug!("Requested users");
            users
        })
        .map_err(|err| err.into())
    }

    pub async fn delete_user_token(&self, token: &str) -> QueriesResult<()> {
        let user = self.get_user(token).await?;

        self.delete_user_user_name(&user.user_name).await?;

        debug!("Deleted user {}", user.user_name);

        Ok(())
    }

    pub async fn delete_user_user_name(&self, user_name: &str) -> QueriesResult<()> {
        query!("DELETE FROM user WHERE user_name = ?", user_name)
            .execute(self.pool)
            .await?;

        debug!("Deleted user {}", user_name);

        Ok(())
    }

    pub async fn update_username(&self, token: &str, new_username: &str) -> QueriesResult<()> {
        let user = self.get_user(token).await?;

        query!(
            "UPDATE user
            SET user_name = $1
            WHERE user_name = $2",
            new_username,
            user.user_name
        )
        .execute(self.pool)
        .await?;

        debug!("Updated user_name {}", new_username);

        Ok(())
    }

    pub async fn update_password(&self, token: &str, new_password: &str) -> QueriesResult<()> {
        let user = self.get_user(token).await?;

        let hashed_password = hash_password(new_password);
        query!(
            "UPDATE user
            SET password_hash = $1
            WHERE user_name = $2",
            hashed_password,
            user.user_name
        )
        .execute(self.pool)
        .await?;

        debug!("Updated password_hash {}", hashed_password);

        Ok(())
    }
}
