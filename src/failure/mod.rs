use core::fmt;
use std::{error::Error, io};

#[derive(Debug)]
pub struct Failure {
    pub reason: String,
}

impl Error for Failure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reason: {}", self.reason)
    }
}

impl From<reqwest::Error> for Failure {
    fn from(reqwest: reqwest::Error) -> Self {
        Failure {
            reason: reqwest.to_string(),
        }
    }
}

impl From<io::Error> for Failure {
    fn from(io: io::Error) -> Self {
        Failure {
            reason: io.to_string(),
        }
    }
}

impl From<serde_json::Error> for Failure {
    fn from(serde_json: serde_json::Error) -> Self {
        Failure {
            reason: serde_json.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Failure>;
