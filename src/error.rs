use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum OsuToolError {
    ConfigError(String),
    IoError(io::Error),
    OsuDbError(String),
    DirectoryNotFound(String),
    DatabaseNotFound(String),
}

impl fmt::Display for OsuToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsuToolError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            OsuToolError::IoError(err) => write!(f, "IO错误: {}", err),
            OsuToolError::OsuDbError(msg) => write!(f, "osu!.db错误: {}", msg),
            OsuToolError::DirectoryNotFound(path) => write!(f, "目录不存在: {}", path),
            OsuToolError::DatabaseNotFound(path) => write!(f, "找不到osu!.db: {}", path),
        }
    }
}

impl Error for OsuToolError {}

impl From<io::Error> for OsuToolError {
    fn from(err: io::Error) -> Self {
        OsuToolError::IoError(err)
    }
}

impl From<toml::de::Error> for OsuToolError {
    fn from(err: toml::de::Error) -> Self {
        OsuToolError::ConfigError(err.to_string())
    }
}

impl From<toml::ser::Error> for OsuToolError {
    fn from(err: toml::ser::Error) -> Self {
        OsuToolError::ConfigError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, OsuToolError>; 