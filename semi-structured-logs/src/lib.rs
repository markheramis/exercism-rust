// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

fn enum_to_str(level: LogLevel) -> String {
    match level {
        LogLevel::Info => {
            return "INFO".to_owned();
        }
        LogLevel::Warning => {
            return "WARNING".to_owned();
        }
        LogLevel::Error => {
            return "ERROR".to_owned();
        }
        LogLevel::Debug => {
            return "DEBUG".to_owned();
        }
    }
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return format!("[{lvl}]: {msg}", lvl = enum_to_str(level), msg = message);
}
pub fn info(message: &str) -> String {
    return format!("[{lvl}]: {msg}", lvl = enum_to_str(LogLevel::Info) , msg = message);
}
pub fn warn(message: &str) -> String {
    return format!("[{lvl}]: {msg}", lvl = enum_to_str(LogLevel::Warning), msg = message);
}
pub fn error(message: &str) -> String {
    return format!("[{lvl}]: {msg}", lvl = enum_to_str(LogLevel::Error), msg = message);
}
pub fn debug(message: &str) -> String {
    return format!("[{lvl}]: {msg}", lvl = enum_to_str(LogLevel::Debug), msg = message);
}
