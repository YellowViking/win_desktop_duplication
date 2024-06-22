use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DDApiError {
    Disconnected,
    Unsupported,
    AccessDenied,
    AccessLost,
    BadParam(String),
    Unexpected(String),
}

impl Display for DDApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DDApiError::Disconnected => write!(f, "Disconnected"),
            DDApiError::Unsupported => write!(f, "Unsupported"),
            DDApiError::AccessDenied => write!(f, "AccessDenied"),
            DDApiError::AccessLost => write!(f, "AccessLost"),
            DDApiError::BadParam(s) => write!(f, "BadParam: {}", s),
            DDApiError::Unexpected(s) => write!(f, "Unexpected: {}", s),
        }
    }
}

impl Error for DDApiError {}