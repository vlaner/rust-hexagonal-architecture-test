use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    NotFound,
    Conflict,
    Validation,
    Unauthorized,
    Forbidden,
    Internal,
}

impl ErrorKind {
    pub fn public_message(self) -> &'static str {
        match self {
            ErrorKind::NotFound => "not found",
            ErrorKind::Conflict => "conflict",
            ErrorKind::Validation => "validation error",
            ErrorKind::Unauthorized => "unauthorized",
            ErrorKind::Forbidden => "forbidden",
            ErrorKind::Internal => "internal server error",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

pub struct AppError {
    pub kind: ErrorKind,
    pub message: Option<&'static str>,
    pub fields: Vec<FieldError>,
    pub cause: Option<anyhow::Error>,
}

impl AppError {
    pub fn not_found() -> Self {
        Self {
            kind: ErrorKind::NotFound,
            message: None,
            fields: Vec::new(),
            cause: None,
        }
    }

    pub fn conflict() -> Self {
        Self {
            kind: ErrorKind::Conflict,
            message: None,
            fields: Vec::new(),
            cause: None,
        }
    }

    pub fn validation(fields: Vec<FieldError>) -> Self {
        Self {
            kind: ErrorKind::Validation,
            message: None,
            fields,
            cause: None,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            kind: ErrorKind::Unauthorized,
            message: None,
            fields: Vec::new(),
            cause: None,
        }
    }

    pub fn forbidden() -> Self {
        Self {
            kind: ErrorKind::Forbidden,
            message: None,
            fields: Vec::new(),
            cause: None,
        }
    }

    pub fn internal(cause: anyhow::Error) -> Self {
        Self {
            kind: ErrorKind::Internal,
            message: None,
            fields: Vec::new(),
            cause: Some(cause),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            self.kind.public_message(),
            self.message.unwrap_or("<empty message>"),
        )
    }
}

impl From<crate::shared::uow::UoWError> for AppError {
    fn from(error: crate::shared::uow::UoWError) -> Self {
        match error {
            crate::shared::uow::UoWError::Begin(cause)
            | crate::shared::uow::UoWError::Commit(cause)
            | crate::shared::uow::UoWError::Rollback(cause) => Self::internal(cause),
        }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // transform chain of errors using colons (like golang wraps)
        let cause_str = self
            .cause
            .as_ref()
            .map(|c| {
                c.chain()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(": ")
            })
            .unwrap_or_else(|| "None".to_string());

        write!(f, "{}", cause_str)
    }
}
