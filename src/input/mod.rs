pub mod clipboard;

use crate::error::InputError;
use clipboard::ClipboardInput;

#[async_trait::async_trait]
pub trait TextInput: Send + Sync {
    /// Get input text, e.g. from clipboard or other sources
    async fn input(&self) -> Result<String, InputError>;

    /// Check if this input method is available
    async fn is_available(&self) -> bool;

    /// Human-readable name for logging
    fn name(&self) -> &'static str;
}

pub fn get_input() -> Result<Box<dyn TextInput>, InputError> {
    Ok(Box::new(ClipboardInput::new()))
}
