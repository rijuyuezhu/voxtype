//! File-based text input
//!
//! Reads text directly from a file path.
//!
//! Useful for processing pre-recorded transcripts or text from other sources.

use super::TextInput;
use crate::error::InputError;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncReadExt;

/// File-based text input
#[derive(Debug)]
pub struct FileInput {
    file_path: PathBuf,
}

impl FileInput {
    /// Create a new file input
    pub fn new(file_path: String) -> Self {
        Self {
            file_path: PathBuf::from(file_path),
        }
    }
}

#[async_trait::async_trait]
impl TextInput for FileInput {
    async fn input(&self) -> Result<String, InputError> {
        // Check if file exists
        if !self.file_path.exists() {
            return Err(InputError::ExtractionFailed(format!(
                "File not found: {}",
                self.file_path.display()
            )));
        }

        // Read file content
        let mut file = fs::File::open(&self.file_path)
            .await
            .map_err(|e| {
                InputError::ExtractionFailed(format!(
                    "Failed to open file {}: {}",
                    self.file_path.display(),
                    e
                ))
            })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(|e| {
                InputError::ExtractionFailed(format!(
                    "Failed to read file {}: {}",
                    self.file_path.display(),
                    e
                ))
            })?;

        let text = String::from_utf8(buffer)
            .map_err(|e| InputError::ExtractionFailed(format!("Invalid UTF-8 in file {}: {}", self.file_path.display(), e)))?;

        tracing::info!("Text read from file {} ({} chars)", self.file_path.display(), text.len());
        Ok(text)
    }

    async fn is_available(&self) -> bool {
        self.file_path.exists()
    }

    fn name(&self) -> &'static str {
        "file"
    }
}
