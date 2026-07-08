use std::sync::Arc;

use sqlx::PgPool;

#[cfg(feature = "manual-uow")]
use crate::modules::auth::service::user::UserService;
use crate::modules::auth::service::user::UserServiceApi;
#[cfg(feature = "callback-uow")]
use crate::modules::auth::service::user::UserServiceCallback;
use crate::shared::postgres::uow::PostgresUnitOfWork;

pub fn create_user_service(pool: PgPool) -> Arc<dyn UserServiceApi> {
    create_user_service_impl(pool)
}

#[cfg(feature = "manual-uow")]
fn create_user_service_impl(pool: PgPool) -> Arc<dyn UserServiceApi> {
    tracing::info!("using manual unit of work");
    let uow = Arc::new(PostgresUnitOfWork::new(pool));

    Arc::new(UserService::new(uow))
}

#[cfg(feature = "callback-uow")]
fn create_user_service_impl(pool: PgPool) -> Arc<dyn UserServiceApi> {
    tracing::info!("using callback unit of work");

    let uow = Arc::new(PostgresUnitOfWork::new(pool));

    Arc::new(UserServiceCallback::new(uow))
}
