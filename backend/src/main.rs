#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate anyhow;

use anyhow::Result;
use api::{api_catchers, api_routes};
use dotenv::dotenv;
use futures::executor::block_on;
use lazy_static::lazy_static;
use sqlx::{Pool, Sqlite};
use std::{env, time::Duration};
use tokio::select;

use crate::models::{create_sqlite_pool, Queries};

mod api;
mod models;

// Necessary because of rockets manage static lifetime requierement
// https://stackoverflow.com/questions/67650879/rust-lazy-static-with-async-await
//
// This will wait until a connection to the db can be created
lazy_static! {
    static ref DB_ADDRESS: String = {
        dotenv().ok();

        env::var("DATABASE_URL").expect("Cant find DATABASE_URL!")
    };
    static ref POOL: Pool<Sqlite> =
        block_on(create_sqlite_pool(&DB_ADDRESS)).expect("Can't connect to db!");
}

#[rocket::main]
async fn main() -> Result<()> {
    // Start both tasks and run them in parallel
    let cleanup = launch_clean_tokens();
    let rocket = launch_rocket();

    // https://www.reddit.com/r/rust/comments/hsekgo/help_how_to_capture_sigint_and_cancel_an_infinite/
    select! {
        result = cleanup => result,
        result = rocket => result
    }
}

async fn launch_rocket() -> Result<()> {
    let queries = Queries::new(&POOL);

    rocket::build()
        .manage(queries)
        .mount("/api", api_routes())
        .register("/api", api_catchers())
        .launch()
        .await?;

    Ok(())
}

// This job will clean up expired tokens
// https://www.reddit.com/r/rust/comments/q0h79p/rocket_and_scheduled_tasks/
async fn launch_clean_tokens() -> Result<()> {
    let queries = Queries::new(&POOL);
    // Run task every minute
    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        queries.auth.cleanup_tokens().await?;
    }
}
