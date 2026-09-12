use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum EaiError {
    Governance(String),
    Hardware(String),
    Protocol(String),
    Inference(String),
    Sandbox(String),
    Config(String),
    Io(String),
    Network(String),
    Filesystem(String),
    Process(String),
    Authentication(String),
    Authorization(String),
    Internal(String),
    #[allow(dead_code)]
    Unknown(Box<dyn StdError + Send + Sync>),
}

impl fmt::Display for EaiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EaiError::Governance(msg) => write!(f, "Governance Violation: {}", msg),
            EaiError::Hardware(msg) => write!(f, "Hardware Error: {}", msg),
            EaiError::Protocol(msg) => write!(f, "Protocol Error: {}", msg),
            EaiError::Inference(msg) => write!(f, "Inference Error: {}", msg),
            EaiError::Sandbox(msg) => write!(f, "Sandbox Error: {}", msg),
            EaiError::Config(msg) => write!(f, "Configuration Error: {}", msg),
            EaiError::Io(msg) => write!(f, "I/O Error: {}", msg),
            EaiError::Network(msg) => write!(f, "Network Error: {}", msg),
            EaiError::Filesystem(msg) => write!(f, "Filesystem Error: {}", msg),
            EaiError::Process(msg) => write!(f, "Process Error: {}", msg),
            EaiError::Authentication(msg) => write!(f, "Authentication Error: {}", msg),
            EaiError::Authorization(msg) => write!(f, "Authorization Error: {}", msg),
            EaiError::Internal(msg) => write!(f, "Internal Engine Error: {}", msg),
            EaiError::Unknown(e) => write!(f, "Unknown Error: {}", e),
        }
    }
}

impl StdError for EaiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            EaiError::Unknown(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EaiError {
    fn from(err: std::io::Error) -> Self {
        EaiError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for EaiError {
    fn from(err: serde_json::Error) -> Self {
        EaiError::Config(format!("JSON error: {}", err))
    }
}

impl From<candle_core::Error> for EaiError {
    fn from(err: candle_core::Error) -> Self {
        EaiError::Inference(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for EaiError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        EaiError::Protocol(format!("UTF-8 error: {}", err))
    }
}

impl From<std::num::ParseIntError> for EaiError {
    fn from(err: std::num::ParseIntError) -> Self {
        EaiError::Protocol(format!("Parse error: {}", err))
    }
}

pub type EaiResult<T> = Result<T, EaiError>;
