use std::fmt;

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR"
        };
        write!(f, "{s}")
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{level}]: {message}")
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, &message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, &message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, &message)
}
