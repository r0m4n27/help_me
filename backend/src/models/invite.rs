use sqlx::{Pool, Sqlite};

use super::{generate_random_string, QueriesResult};

#[derive(Debug, FromRow, Serialize)]
pub struct Invite {
    pub invite_code: String,
}

pub struct InviteQueries<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> InviteQueries<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        InviteQueries { pool }
    }

    pub async fn get_invites(&self) -> QueriesResult<Vec<Invite>> {
        let invites = query_as!(Invite, "SELECT * FROM invite")
            .fetch_all(self.pool)
            .await?;

        Ok(invites)
    }

    pub async fn delete_invite(&self, invite_code: &str) -> QueriesResult<()> {
        query!(
            "DELETE FROM invite
            WHERE invite_code = ?",
            invite_code
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_invite(&self) -> QueriesResult<Invite> {
        let invite_code = generate_random_string(12);

        query!(
            "INSERT INTO invite(invite_code)
            VALUES (?)",
            invite_code
        )
        .execute(self.pool)
        .await?;

        Ok(Invite { invite_code })
    }

    pub async fn invite_exists(&self, invite_code: &str) -> QueriesResult<bool> {
        let invite = query_as!(
            Invite,
            "SELECT * FROM invite
            WHERE invite_code = ?",
            invite_code
        )
        .fetch_optional(self.pool)
        .await?;

        match invite {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
