pub mod domain;
pub mod postgres;

use std::{net::TcpListener, time::Duration};
use std::sync::Arc;
use actix_web::{Responder, error::ResponseError};
use actix_web::http::StatusCode;
use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use crate::domain::audit::AuditError;
use crate::domain::uow::{UnitOfWork, UoWError};
use crate::domain::users::UserError;
use crate::postgres::uow::PostgresUnitOfWork;

pub struct AppState {
    pub uow: Arc<dyn UnitOfWork>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("internal server error")]
    Internal(#[source] anyhow::Error),
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}


impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict => StatusCode::CONFLICT,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse{error: self.to_string()})
    }
}

impl From<UserError> for ApiError {
    fn from(e: UserError) -> Self {
        match e {
            UserError::NotFound(_) => ApiError::NotFound,
            UserError::DuplicateUsername(_) => ApiError::Conflict,
            UserError::Unknown(e) => ApiError::Internal(e),
        }
    }
}

impl From<AuditError> for ApiError {
    fn from(e: AuditError) -> Self {
        match e {
            AuditError::Unknown(e) => ApiError::Internal(e),
        }
    }
}

impl From<UoWError> for ApiError {
    fn from(e: UoWError) -> Self {
        match e {
            UoWError::Unknown(e) => ApiError::Internal(e),
        }
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
    created_at: DateTime<Utc>
}

async fn index(
    state: web::Data<AppState>,
    query: web::Query<UidParam>,
) -> Result<impl Responder, ApiError> {
    let uid = query.into_inner().uid;
    let mut tx = state.uow.begin().await?;
    let user = tx.user().by_id(uid).await?;
    let _ = tx.audit().log(uid, "read_user").await?;
    tx.commit().await?;
    
    Ok(web::Json(UserResponse{ id: user.id, username:  user.username, created_at: user.created_at }))
}

pub async fn run(listener: TcpListener) -> anyhow::Result<Server> {
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3)) 
        .acquire_slow_threshold(Duration::from_secs(2))
        .connect(&database_url)
        .await
        .context("connect to postgres")?;


    let uow: Arc<dyn UnitOfWork> = Arc::new(PostgresUnitOfWork::new(pool));
    let app_state = web::Data::new(AppState { uow });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
