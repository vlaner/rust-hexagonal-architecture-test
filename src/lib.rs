pub mod domain;
pub mod error;
pub mod postgres;
pub mod service;

use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, dev::Server, error::ResponseError, http::StatusCode,
    web,
};
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

use crate::domain::uow::UnitOfWork;
use crate::error::{AppError, ErrorKind};
use crate::postgres::uow::PostgresUnitOfWork;
use crate::service::UserService;

pub struct AppState {
    pub user_service: Arc<UserService>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<crate::error::FieldError>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Conflict => StatusCode::CONFLICT,
            ErrorKind::Validation => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorKind::Forbidden => StatusCode::FORBIDDEN,
            ErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self
                .message
                .unwrap_or(self.kind.public_message())
                .to_string(),
            fields: self.fields.clone(),
        })
    }
}

#[derive(serde::Deserialize)]
struct UidParam {
    uid: uuid::Uuid,
}

#[derive(serde::Serialize)]
struct UserResponse {
    id: uuid::Uuid,
    username: String,
    created_at: DateTime<Utc>,
}

async fn index(
    state: web::Data<AppState>,
    query: web::Query<UidParam>,
) -> Result<impl Responder, AppError> {
    let user = state.user_service.get_user(query.uid).await?;

    Ok(web::Json(UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
    }))
}

pub async fn run(listener: TcpListener) -> anyhow::Result<Server> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_backend=debug,actix_web=info,tracing_actix_web=debug".into()
            }),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();
    tracing::info!("logging initialized");

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .acquire_slow_threshold(Duration::from_secs(2))
        .connect(&database_url)
        .await
        .context("connect to postgres")?;

    let uow: Arc<dyn UnitOfWork> = Arc::new(PostgresUnitOfWork::new(pool.clone()));
    let user_service = Arc::new(UserService::new(uow));
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
