use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TransactionError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Invalid input")]
    InvalidInput,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid input: {field}")]
    InvalidInput { field: String },
    #[error("User with id not found")]
    NotFound,
    #[error("User already exists: {1} ({0})")]
    AlreadyExists(i32, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Service error")]
    BadService,
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Another custom error: {0}")]
    AnotherCustom(String),
    #[error(transparent)]
    Transaction(#[from] TransactionError),
}

impl Error {
    pub fn retryable(&self) -> bool {
        matches!(self, Self::Io(_) | Self::BadService | Self::Custom(_))
    }

    pub fn severity_level(&self) -> SeverityLevel {
        match self {
            Self::InvalidInput { .. } => SeverityLevel::Warning,
            Self::Transaction(TransactionError::InvalidInput) => SeverityLevel::Info,
            _ => SeverityLevel::Error,
        }
    }
}
