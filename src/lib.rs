pub mod domain;
pub mod postgres;

use crate::domain::uow::UnitOfWork;
use crate::postgres::uow::PostgresUnitOfWork;
use actix_web::{App,  HttpServer, dev::Server, web};
use actix_web::{Responder, error::ResponseError};
use std::net::TcpListener;
use std::sync::Arc;
use anyhow::{Context, Result};

pub struct AppState {
    pub uow: Arc<dyn UnitOfWork>,
}

impl ResponseError for crate::domain::users::UserError {}
impl ResponseError for crate::domain::audit::AuditError {}
impl ResponseError for crate::domain::uow::UoWError {}

#[derive(serde::Deserialize)]
struct UidParam {
    uid: uuid::Uuid,
}

async fn index(
    state: web::Data<AppState>,
    query: web::Query<UidParam>,
) -> Result<impl Responder, actix_web::Error> {
    let uid = query.into_inner().uid;

    let mut tx = state.uow.begin().await?;
    let user = tx.user().by_id(uid).await?;
    tx.audit().log(uid, "read_user").await?;
    tx.commit().await?;

    Ok(format!("{:?}", user))
}

pub async fn run(listener: TcpListener) -> Result<Server> {
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is required")?;

    let pool = sqlx::PgPool::connect(&database_url)
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
