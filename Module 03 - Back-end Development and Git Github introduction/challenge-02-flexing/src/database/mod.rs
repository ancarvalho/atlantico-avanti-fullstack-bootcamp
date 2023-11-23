use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use anyhow::{Result, Ok};

pub async fn connect() -> Result<Pool<Postgres>> {


  let database_url =
    env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:postgres@localhost:5432/challenge-02"));

  let database_pool_size = env::var("DATABASE_POOL")
    .ok()
    .and_then(|port| port.parse::<u32>().ok())
    .unwrap_or(30);

  let pool = PgPoolOptions::new()
    .max_connections(database_pool_size)
    .connect(&database_url)
    .await?;

 
  Ok(pool)
}
