use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

mod config;
mod http;

use config::Config;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "not_so_realworld_server=DEBUG,tower_http=DEBUG".into());
    let file_appender = tracing_appender::rolling::never("logs", "example.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let non_blocking_layer = Layer::new().with_writer(non_blocking).with_ansi(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(non_blocking_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&db).await.context("Migrate error")?;

    http::serve(config, db).await
}
