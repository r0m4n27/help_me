use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use self::{auth::AuthQueries, invite::InviteQueries};

mod auth;
pub mod invite;

#[derive(Debug, FromRow)]
pub struct User {
    user_name: String,
    password_hash: String,
    user_type: String,
}

pub enum UserType {
    Admin,
    Tutor,
}

impl ToString for UserType {
    fn to_string(&self) -> String {
        match self {
            UserType::Admin => String::from("admin"),
            UserType::Tutor => String::from("tutor"),
        }
    }
}

pub struct Queries {
    //pool: Pool<Sqlite>,
    // The static is necessary for rocket
    pub auth: AuthQueries<'static>,
    pub invite: InviteQueries<'static>,
}

impl Queries {
    pub fn new(pool: &'static Pool<Sqlite>) -> Queries {
        Queries {
            auth: AuthQueries::new(pool),
            invite: InviteQueries::new(pool),
        }
    }
}

pub async fn create_sqlite_pool(connection_url: &str) -> Result<Pool<Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(connection_url)
        .await?;

    Ok(pool)
}

// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
fn generate_random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
