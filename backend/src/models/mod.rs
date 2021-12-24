use blake2::{Blake2b512, Digest};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::{self, sqlite::SqlitePoolOptions, Pool, Sqlite};

use self::{auth::AuthQueries, invite::InviteQueries, task::TaskQueries, user::UserQueries};

mod auth;
mod invite;
mod task;
mod user;

pub use invite::Invite;
pub use task::Task;

#[derive(Error, Debug)]
pub enum QueriesError {
    #[error("Error with the database occured: {0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    ItemNotFound(String),
    #[error("{0}")]
    IllegalState(String),
}

type QueriesResult<T> = Result<T, QueriesError>;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub user_name: String,
    #[serde(skip_serializing)]
    password_hash: String,
    pub user_type: String,
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
    pub user: UserQueries<'static>,
    pub task: TaskQueries<'static>,
}

impl Queries {
    pub fn new(pool: &'static Pool<Sqlite>) -> Queries {
        Queries {
            auth: AuthQueries::new(pool),
            invite: InviteQueries::new(pool),
            user: UserQueries::new(pool),
            task: TaskQueries::new(pool),
        }
    }
}

// In this version of sqlx it is not possible to use Pool<Any>
// because the query macros will create a specific instance
// and it is not yet possible to specify the driver to any
//
// https://github.com/launchbadge/sqlx/issues/964
pub async fn create_sqlite_pool(connection_url: &str) -> Result<Pool<Sqlite>, QueriesError> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(connection_url)
        .await?;

    info!("Created poll for {}", connection_url);

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

fn hash_password(password: &str) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(password.as_bytes());

    format!("{:x}", hasher.finalize())
}
