use crate::{
    contracts::audit::{audit::AuditRepository, uow::HasAuditRepo},
    modules::audit::postgres::audit::PostgresAuditRepoTx,
    shared::postgres::uow::PostgresUnitOfWorkTransaction,
};

impl HasAuditRepo for PostgresUnitOfWorkTransaction {
    fn audit(&mut self) -> impl AuditRepository + '_ {
        PostgresAuditRepoTx::new(&mut self.tx)
    }
}
