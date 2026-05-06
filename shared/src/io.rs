use super::StarterError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to create file '{path}': {source}")]
    CreateFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to read file '{path}': {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write to file '{path}': {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to append to file '{path}': {source}")]
    AppendFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to delete file '{path}': {source}")]
    DeleteFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to create directory '{path}': {source}")]
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to delete directory '{path}': {source}")]
    DeleteDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to copy file from '{from}' to '{to}': {source}")]
    CopyFile {
        from: PathBuf,
        to: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to move file from '{from}' to '{to}': {source}")]
    MoveFile {
        from: PathBuf,
        to: PathBuf,
        source: std::io::Error,
    },

    #[error("File not found: '{path}'")]
    FileNotFound { path: PathBuf },

    #[error("Permission denied: '{path}'")]
    PermissionDenied { path: PathBuf },

    #[error("Command Failed: '{command}'")]
    CommandFailed {
        command: String,
        source: std::io::Error,
    },
}

impl StarterError for IoError {
    fn code(&self) -> u32 {
        match self {
            IoError::CreateFile { .. } => 5001,
            IoError::ReadFile { .. } => 5002,
            IoError::WriteFile { .. } => 5003,
            IoError::AppendFile { .. } => 5004,
            IoError::DeleteFile { .. } => 5005,
            IoError::CreateDir { .. } => 5006,
            IoError::DeleteDir { .. } => 5007,
            IoError::CopyFile { .. } => 5008,
            IoError::MoveFile { .. } => 5009,
            IoError::FileNotFound { .. } => 5010,
            IoError::PermissionDenied { .. } => 5011,
            IoError::CommandFailed { .. } => 5012,
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }

    fn hint(&self) -> Option<String> {
        match self {
            IoError::FileNotFound { path } => Some(format!(
                "Check if '{}' exists and the path is correct",
                path.display()
            )),
            IoError::PermissionDenied { path } => {
                Some(format!("Check file permissions for '{}'", path.display()))
            }
            IoError::CreateFile { path, .. } => Some(format!(
                "Ensure the parent directory exists and you have write permission for '{}'",
                path.display()
            )),
            _ => None,
        }
    }
}

pub type IoResult<T> = Result<T, IoError>;
