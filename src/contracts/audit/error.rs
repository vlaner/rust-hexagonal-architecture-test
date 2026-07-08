use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("unexpected audit error")]
    Unknown(#[from] anyhow::Error),
}
