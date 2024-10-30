use std::fmt::{Debug, Display};

pub enum AkinatorError {
    SessionNotFound,
    ConnectionError,
    QuestionLimitExceeded,
    InvalidResponse,
    UnexpectedError,
    CannotBackAnyMore,
}

impl Debug for AkinatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AkinatorError::SessionNotFound => write!(f, "Session not found"),
            AkinatorError::ConnectionError => write!(f, "Connection error"),
            AkinatorError::QuestionLimitExceeded => write!(f, "Question limit exceeded"),
            AkinatorError::InvalidResponse => write!(f, "Invalid response"),
            AkinatorError::UnexpectedError => write!(f, "Unexpected error"),
            AkinatorError::CannotBackAnyMore => write!(f, "Cannot back any more"),
        }
    }
}

impl Display for AkinatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
