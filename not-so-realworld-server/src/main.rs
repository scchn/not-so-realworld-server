use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

mod config;
mod http;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&db).await.context("Migrate error")?;

    http::serve(config, db).await
}
