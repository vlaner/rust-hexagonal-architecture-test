mod domain;
pub mod http;
pub mod postgres;
mod service;
mod uow;

use std::sync::Arc;

use super::api::{HasAuditPort, UserServiceApi};
use crate::shared::uow::{UnitOfWork, UnitOfWorkCallback};

use self::service::user::{UserService, UserServiceCallback};

pub use super::api::{AuditPort, AuditPortError};
pub use domain::user::UserRepository;
pub use uow::types::HasUserRepo;

pub fn create_user_service_manual<U>(uow: Arc<U>) -> Arc<dyn UserServiceApi>
where
    U: UnitOfWork + Send + Sync + 'static,
    U::Tx: HasUserRepo + HasAuditPort,
{
    Arc::new(UserService::new(uow))
}

pub fn create_user_service_callback<U>(uow: Arc<U>) -> Arc<dyn UserServiceApi>
where
    U: UnitOfWorkCallback + Send + Sync + 'static,
    U::Tx: HasUserRepo + HasAuditPort,
{
    Arc::new(UserServiceCallback::new(uow))
}
