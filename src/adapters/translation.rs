//! This module exposes the adapter for translating text using an external translation service.

mod fun_translations;
#[cfg(test)]
mod mock;

pub use self::fun_translations::FunTranslationsAdapter;
#[cfg(test)]
pub use self::mock::MockTranslationAdapter;
use crate::model::Pokemon;

/// Result type for [`TranslationAdapter`] operations
pub type TranslationAdapterResult<T> = Result<T, TranslationAdapterError>;

/// Errors that can occur while interacting with the [`TranslationAdapter`]
#[derive(Debug, thiserror::Error)]
pub enum TranslationAdapterError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("Unexpected response from the API: {0}")]
    UnexpectedResponse(String),
}

/// The `TranslationAdapter` trait defines the interface for translating text into different styles.
pub trait TranslationAdapter {
    /// Translates the given text into Shakespearean-style speech.
    fn translate_into_shakespeare(
        &self,
        text: &str,
    ) -> impl Future<Output = TranslationAdapterResult<String>>;

    /// Translates the given text into Yoda-style speech.
    fn translate_into_yoda(
        &self,
        text: &str,
    ) -> impl Future<Output = TranslationAdapterResult<String>>;
}
