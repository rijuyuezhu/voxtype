pub mod clipboard;
pub mod file_input;

use crate::error::InputError;
use clipboard::ClipboardInput;
use file_input::FileInput;

/// Trait for text input methods, e.g. clipboard, file, etc.
#[async_trait::async_trait]
pub trait TextInput: Send + Sync {
    /// Get input text, e.g. from clipboard or other sources
    async fn input(&self) -> Result<String, InputError>;

    /// Check if this input method is available
    async fn is_available(&self) -> bool;

    /// Human-readable name for logging
    fn name(&self) -> &'static str;
}

pub fn get_input(source: Option<String>) -> Result<Box<dyn TextInput>, InputError> {
    if let Some(input_file) = source {
        Ok(Box::new(FileInput::new(input_file)))
    } else {
        Ok(Box::new(ClipboardInput::new()))
    }
}
