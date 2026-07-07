use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    #[error("username already exists")]
    DuplicateUsername,
    #[error("unexpected user error")]
    Unknown(#[from] anyhow::Error),
}
