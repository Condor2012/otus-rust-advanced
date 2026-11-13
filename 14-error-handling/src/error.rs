use std::error::Error;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub enum GetDataError {
    Login(LoginError),
    Download(DownloadError),
}

impl Error for GetDataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GetDataError::Login(e) => Some(e),
            GetDataError::Download(e) => Some(e),
        }
    }
}

// impl Debug for GetDataError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Login(e) => write!(f, "GetDataError::Login({e})"),
//             Self::Download(e) => write!(f, "GetDataError::Download({e})"),
//         }
//     }
// }

impl Display for GetDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Login(_) => write!(f, "failed to login"),
            Self::Download(_) => write!(f, "failed to download data"),
        }
    }
}

impl From<LoginError> for GetDataError {
    fn from(e: LoginError) -> Self {
        GetDataError::Login(e)
    }
}


impl From<DownloadError> for GetDataError {
    fn from(e: DownloadError) -> Self {
        GetDataError::Download(e)
    }
}

#[derive(Debug)]
pub enum LoginError {
    IncorrectEmail,
    IncorrectPassword,
}

impl Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectEmail => write!(f, "incorrect email"),
            Self::IncorrectPassword => write!(f, "incorrect password"),
        }
    }
}

impl Error for LoginError {}

#[derive(Debug)]
pub enum DownloadError {
    IncorrectUserId,
    IncorrectKey,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectUserId => write!(f, "incorrect user id"),
            Self::IncorrectKey => write!(f, "incorrect key"),
        }
    }
}

impl Error for DownloadError {}
