use std::fmt::Display;
use std::env::VarError;

#[derive(Debug)]
pub enum JMAPError {
    VarError(VarError),
    Error(reqwest::Error)
}

impl Display for JMAPError {
    fn fmt(&self, f: & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JMAPError::Error(e) => {
                write!(f, "{}", e)
            },
            JMAPError::VarError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl From<reqwest::Error> for JMAPError {
    fn from(err: reqwest::Error) -> Self {
        JMAPError::Error(err)
    }
}

impl From<std::env::VarError> for JMAPError {
    fn from(err: VarError) -> Self {
        JMAPError::VarError(err)
    }
}

impl std::error::Error for JMAPError {}
