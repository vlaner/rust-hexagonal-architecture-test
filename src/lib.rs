pub mod features;
pub mod modules;
pub mod shared;

use std::time::Duration;
use std::{net::TcpListener, sync::Arc};

use crate::shared::postgres::uow::PostgresUnitOfWork;
use actix_web::{App, HttpServer, dev::Server, web};
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

use crate::modules::auth::core::http::{AppState, index};

pub async fn run(listener: TcpListener) -> anyhow::Result<Server> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "rust_backend=debug,actix_web=info,tracing_actix_web=debug,sqlx=debug".into()
        }))
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .acquire_slow_threshold(Duration::from_secs(2))
        .connect(&database_url)
        .await
        .context("connect to postgres")?;

    let uow = Arc::new(PostgresUnitOfWork::new(pool));
    let user_service = features::create_user_service_impl(uow);
    let app_state = web::Data::new(AppState { user_service });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            // TODO: duplicate ERROR and INFO levels with same fields
            .wrap(TracingLogger::default())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
