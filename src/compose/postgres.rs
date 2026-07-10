use crate::modules::audit::core::domain::audit::AuditRepository;
use crate::{
    modules::{
        audit::{
            api::AuditApi,
            core::postgres::audit::PostgresAuditRepoTx,
        },
        auth::core::{
            HasUserRepo, UserRepository,
            audit_port::{AuditPort, HasAuditPort},
            postgres::user::PostgresUserRepoTx,
        },
    },
    shared::{apperror::AppError, postgres::uow::PostgresUnitOfWorkTransaction},
};

impl HasUserRepo for PostgresUnitOfWorkTransaction {
    fn user(&mut self) -> impl UserRepository + '_ {
        PostgresUserRepoTx::new(&mut self.tx)
    }
}

struct AuthAuditPort<'a, 'c> {
    audit: PostgresAuditRepoTx<'a, 'c>,
}

#[async_trait::async_trait]
impl AuditPort for AuthAuditPort<'_, '_> {
    async fn log(&mut self, user_id: uuid::Uuid, action: &str) -> Result<(), AppError> {
        let _ = self.audit.log(user_id, action).await?;
        Ok(())
    }
}

// adapt user defined audit port to audit transaction boundary
impl HasAuditPort for PostgresUnitOfWorkTransaction {
    fn audit(&mut self) -> impl AuditPort + '_ {
        AuthAuditPort {
            audit: PostgresAuditRepoTx::new(&mut self.tx),
        }
    }
}
