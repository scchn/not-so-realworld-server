use anyhow::Context;
use axum::{extract::FromRef, http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

mod extractors;
mod response;
mod traits;
mod user;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(1);
const SCHEME_PREFIX: &str = "Bearer ";

pub type Result<T, E = response::ApiError> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct AppState {
    config: Config,
    db: PgPool,
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));
    let state = AppState { config, db };
    let app = Router::new()
        .fallback(handler_not_found)
        .route("/", get(handler_root))
        .nest("/api/user", user::router())
        .layer(trace_layer)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:4000").await?;

    axum::serve(listener, app)
        .await
        .context("error running server")
}

async fn handler_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT FOUND")
}

async fn handler_root() -> impl IntoResponse {
    "Welcome to Not-so-realworld server"
}
