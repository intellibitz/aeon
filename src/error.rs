// AEON Unified Error Protocol
// Categorized error handling for Exponential Intelligence Substrate

use std::fmt;

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
    #[allow(dead_code)]
    Internal(String),
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
        }
    }
}

impl std::error::Error for EaiError {}

impl From<std::io::Error> for EaiError {
    fn from(err: std::io::Error) -> Self {
        EaiError::Io(err.to_string())
    }
}

impl From<candle_core::Error> for EaiError {
    fn from(err: candle_core::Error) -> Self {
        EaiError::Inference(err.to_string())
    }
}

pub type EaiResult<T> = Result<T, EaiError>;
