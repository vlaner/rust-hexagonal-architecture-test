pub mod audit_port;
mod domain;
pub mod http;
pub mod postgres;
mod service;
mod uow;

use std::sync::Arc;

use self::audit_port::HasAuditPort;
use super::api::UserServiceApi;
use crate::shared::uow::{UnitOfWork, UnitOfWorkCallback};

use self::service::user::{UserService, UserServiceCallback};

pub use audit_port::AuditPort;
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
