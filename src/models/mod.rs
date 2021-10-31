use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use self::auth::AuthQueries;

pub mod auth;

#[derive(Debug, FromRow)]
pub struct User {
    user_name: String,
    password_hash: String,
    user_type: String,
}

pub enum UserType {
    Admin,
}

impl ToString for UserType {
    fn to_string(&self) -> String {
        match self {
            UserType::Admin => String::from("admin"),
        }
    }
}

pub struct Queries {
    //pool: Pool<Sqlite>,
    // The static is necessary for rocket
    pub auth: AuthQueries<'static>,
}

impl Queries {
    pub fn new(pool: &'static Pool<Sqlite>) -> Queries {
        Queries {
            auth: AuthQueries::new(pool),
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
