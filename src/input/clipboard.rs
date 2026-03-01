//! Clipboard-based text input
//!
//! Uses wl-paste to read text from the Wayland clipboard.
//!
//! Requires: wl-clipboard package installed

use super::TextInput;
use crate::error::InputError;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

/// Clipboard-based text input
#[derive(Debug, Default)]
pub struct ClipboardInput;

impl ClipboardInput {
    /// Create a new clipboard input
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl TextInput for ClipboardInput {
    async fn input(&self) -> Result<String, InputError> {
        // Spawn wl-paste with stdout pipe
        let mut child = Command::new("wl-paste")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    InputError::WlPasteNotFound
                } else {
                    InputError::ExtractionFailed(e.to_string())
                }
            })?;

        // Read text from stdout
        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| InputError::ExtractionFailed("Failed to capture stdout".to_string()))?;

        let mut buffer = Vec::new();
        stdout
            .read_to_end(&mut buffer)
            .await
            .map_err(|e| InputError::ExtractionFailed(e.to_string()))?;

        let text = String::from_utf8(buffer)
            .map_err(|e| InputError::ExtractionFailed(format!("Invalid UTF-8: {}", e)))?;

        // Wait for completion
        let status = child
            .wait()
            .await
            .map_err(|e| InputError::ExtractionFailed(e.to_string()))?;

        if !status.success() {
            return Err(InputError::ExtractionFailed(
                "wl-paste exited with error".to_string(),
            ));
        }

        // Remove trailing newline that wl-paste often adds
        let text = text.trim_end_matches('\n').to_string();

        tracing::info!("Text read from clipboard ({} chars)", text.len());
        Ok(text)
    }

    async fn is_available(&self) -> bool {
        Command::new("which")
            .arg("wl-paste")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "clipboard (wl-paste)"
    }
}