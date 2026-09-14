use std::fmt;
use std::error::Error as StdError;
use std::backtrace::Backtrace;


#[derive(Debug)]
pub enum EaiError {
    Governance(String, Backtrace),
    Hardware(String, Backtrace),
    Protocol(String, Backtrace),
    Inference(String, Backtrace),
    Sandbox(String, Backtrace),
    Config(String, Backtrace),
    Io(String, Backtrace),
    Network(String, Backtrace),
    Filesystem(String, Backtrace),
    Process(String, Backtrace),
    Authentication(String, Backtrace),
    Authorization(String, Backtrace),
    Internal(String, Backtrace),
    #[allow(dead_code)]
    Unknown(Box<dyn StdError + Send + Sync>, Backtrace),
}

impl EaiError {
    pub fn governance(msg: impl Into<String>) -> Self {
        let e = EaiError::Governance(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn hardware(msg: impl Into<String>) -> Self {
        let e = EaiError::Hardware(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn protocol(msg: impl Into<String>) -> Self {
        let e = EaiError::Protocol(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn inference(msg: impl Into<String>) -> Self {
        let e = EaiError::Inference(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn sandbox(msg: impl Into<String>) -> Self {
        let e = EaiError::Sandbox(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn config(msg: impl Into<String>) -> Self {
        let e = EaiError::Config(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn io(msg: impl Into<String>) -> Self {
        let e = EaiError::Io(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn network(msg: impl Into<String>) -> Self {
        let e = EaiError::Network(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn filesystem(msg: impl Into<String>) -> Self {
        let e = EaiError::Filesystem(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn process(msg: impl Into<String>) -> Self {
        let e = EaiError::Process(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn authentication(msg: impl Into<String>) -> Self {
        let e = EaiError::Authentication(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn authorization(msg: impl Into<String>) -> Self {
        let e = EaiError::Authorization(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        let e = EaiError::Internal(msg.into(), Backtrace::capture());
        e.log_to_metrics();
        e
    }

    fn log_to_metrics(&self) {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let metrics_file = home.join(".susi/error_metrics.jsonl");
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);

        let entry = serde_json::json!({
            "ts": ts,
            "error": self.to_string(),
            "kind": self.kind_name(),
        });

        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(metrics_file) {
            use std::io::Write;
            let _ = writeln!(f, "{}", entry);
        }
    }

    fn kind_name(&self) -> &'static str {
        match self {
            EaiError::Governance(_, _) => "Governance",
            EaiError::Hardware(_, _) => "Hardware",
            EaiError::Protocol(_, _) => "Protocol",
            EaiError::Inference(_, _) => "Inference",
            EaiError::Sandbox(_, _) => "Sandbox",
            EaiError::Config(_, _) => "Config",
            EaiError::Io(_, _) => "Io",
            EaiError::Network(_, _) => "Network",
            EaiError::Filesystem(_, _) => "Filesystem",
            EaiError::Process(_, _) => "Process",
            EaiError::Authentication(_, _) => "Authentication",
            EaiError::Authorization(_, _) => "Authorization",
            EaiError::Internal(_, _) => "Internal",
            EaiError::Unknown(_, _) => "Unknown",
        }
    }
}

impl fmt::Display for EaiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EaiError::Governance(msg, _) => write!(f, "Governance Violation: {}", msg),
            EaiError::Hardware(msg, _) => write!(f, "Hardware Error: {}", msg),
            EaiError::Protocol(msg, _) => write!(f, "Protocol Error: {}", msg),
            EaiError::Inference(msg, _) => write!(f, "Inference Error: {}", msg),
            EaiError::Sandbox(msg, _) => write!(f, "Sandbox Error: {}", msg),
            EaiError::Config(msg, _) => write!(f, "Configuration Error: {}", msg),
            EaiError::Io(msg, _) => write!(f, "I/O Error: {}", msg),
            EaiError::Network(msg, _) => write!(f, "Network Error: {}", msg),
            EaiError::Filesystem(msg, _) => write!(f, "Filesystem Error: {}", msg),
            EaiError::Process(msg, _) => write!(f, "Process Error: {}", msg),
            EaiError::Authentication(msg, _) => write!(f, "Authentication Error: {}", msg),
            EaiError::Authorization(msg, _) => write!(f, "Authorization Error: {}", msg),
            EaiError::Internal(msg, _) => write!(f, "Internal Engine Error: {}", msg),
            EaiError::Unknown(e, _) => write!(f, "Unknown Error: {}", e),
        }
    }
}

impl StdError for EaiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            EaiError::Unknown(e, _) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EaiError {
    fn from(err: std::io::Error) -> Self {
        let e = EaiError::Io(err.to_string(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
}

impl From<serde_json::Error> for EaiError {
    fn from(err: serde_json::Error) -> Self {
        let e = EaiError::Config(format!("JSON error: {}", err), Backtrace::capture());
        e.log_to_metrics();
        e
    }
}

impl From<candle_core::Error> for EaiError {
    fn from(err: candle_core::Error) -> Self {
        let e = EaiError::Inference(err.to_string(), Backtrace::capture());
        e.log_to_metrics();
        e
    }
}

impl From<std::string::FromUtf8Error> for EaiError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        let e = EaiError::Protocol(format!("UTF-8 error: {}", err), Backtrace::capture());
        e.log_to_metrics();
        e
    }
}

impl From<std::num::ParseIntError> for EaiError {
    fn from(err: std::num::ParseIntError) -> Self {
        let e = EaiError::Protocol(format!("Parse error: {}", err), Backtrace::capture());
        e.log_to_metrics();
        e
    }
}

pub type EaiResult<T> = Result<T, EaiError>;
