use thiserror::Error;
use std::fmt;
use derive_more::{Display, From};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Error, Display, From)]
#[non_exhaustive]
pub enum TransactionError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Invalid input")]
    InvalidInput,
}


#[derive(Debug, Error, Display, From)]
#[non_exhaustive]
pub enum UserError<MyType> {
    #[error("Invalid input: {field}")]
    InvalidInput { field: MyType },
    #[error("User with id not found")]
    NotFound,
    #[error("IO error: {1}")]
    AlreadyExists (i32, String),
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[error("Service error")]
    BadService,
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Another custom error: {0}")]
    AnotherCustom(String),
}

impl Error {
    pub fn retryable(&self) -> bool {
        match self {
            Error::IoError(_) => true,
            Error::BadService => true,
            Error::Custom(_) => true,
            _ => false,
        }
    }    

    pub fn severity_level(&self) -> SeverityLevel {
        match self {
            Error::InvalidInput { .. } => SeverityLevel::Warning,
            Error::NotFound => SeverityLevel::Error,
            Error::AlreadyExists(_, _) => SeverityLevel::Error,
            Error::IoError(_) => SeverityLevel::Error,
            Error::BadService => SeverityLevel::Error,
            Error::Custom(_) => SeverityLevel::Error,
            Error::AnotherCustom(_) => SeverityLevel::Error,
        }
    }
}

pub enum SystemError {
    ServiceError(actix_web::Error),
}

impl From<TransactionError> for Error {
    fn from(error: TransactionError) -> Self {
        Error::TransactionError(error)
    }
}
