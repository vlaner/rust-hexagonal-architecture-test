use std::sync::Arc;

use crate::{
    modules::auth::{api::UserServiceApi, core},
    shared::postgres::uow::PostgresUnitOfWork,
};

#[cfg(feature = "manual-uow")]
pub(crate) fn create_user_service_impl(uow: Arc<PostgresUnitOfWork>) -> Arc<dyn UserServiceApi> {
    tracing::info!("using manual unit of work");
    core::create_user_service_manual(uow)
}

#[cfg(all(feature = "callback-uow", not(feature = "manual-uow")))]
pub(crate) fn create_user_service_impl(uow: Arc<PostgresUnitOfWork>) -> Arc<dyn UserServiceApi> {
    tracing::info!("using callback unit of work");
    core::create_user_service_callback(uow)
}
