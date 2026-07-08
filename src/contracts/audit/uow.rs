use crate::contracts::audit::audit::AuditRepository;

pub trait HasAuditRepo {
    fn audit(&mut self) -> impl AuditRepository + '_;
}
