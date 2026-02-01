//! Session data loading from exported files.

use crate::Session;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Session file not found: {0}")]
    NotFound(String),
}

/// Load a session from a JSON file.
pub fn load_session(path: impl AsRef<Path>) -> Result<Session, LoadError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(LoadError::NotFound(path.display().to_string()));
    }
    let contents = std::fs::read_to_string(path)?;
    let session: Session = serde_json::from_str(&contents)?;
    Ok(session)
}

/// Load a session from JSON string.
pub fn load_session_from_str(json: &str) -> Result<Session, LoadError> {
    let session: Session = serde_json::from_str(json)?;
    Ok(session)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_session("/nonexistent/path.json");
        assert!(matches!(result, Err(LoadError::NotFound(_))));
    }
}
