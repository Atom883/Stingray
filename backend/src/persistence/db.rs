use anyhow::Result;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::str::FromStr as _;

const DB_NAME: &str = "db.sqlite";

pub async fn create_sqlite_pool(dir: &str) -> Result<SqlitePool> {
    let database_dir = dir.replace('\\', "/");
    let database_url = format!("sqlite://{database_dir}/{DB_NAME}");

    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!("./src/persistence/migrations")
        .run(&sqlite_pool)
        .await?;

    Ok(sqlite_pool)
}
