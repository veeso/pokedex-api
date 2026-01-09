use std::time::Duration;

use url::Url;

use crate::adapters::translation::fun_translations::response::TranslateResponse;
use crate::adapters::translation::{
    TranslationAdapter, TranslationAdapterError, TranslationAdapterResult,
};

mod response;

const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(30);

/// Adapter for Fun Translations API
pub struct FunTranslationsAdapter {
    client: reqwest::Client,
    endpoint: Url,
    timeout: Duration,
}

impl Default for FunTranslationsAdapter {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: "https://api.funtranslations.com/translate/"
                .parse()
                .expect("valid URL"),
            timeout: DEFAULT_API_TIMEOUT,
        }
    }
}

impl FunTranslationsAdapter {
    /// Create a new instance of [`FunTranslationsAdapter`] with the given endpoint and timeout.
    pub fn new(endpoint: impl Into<Url>, timeout: Duration) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: endpoint.into(),
            timeout,
        }
    }

    /// Translate the given text using the specified translation URL.
    async fn translate(&self, url: Url, text: &str) -> TranslationAdapterResult<String> {
        let response = self
            .client
            .post(url)
            .form(&[("text", text)])
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|err| TranslationAdapterError::NetworkError(err.to_string()))?;

        if !response.status().is_success() {
            error!(
                "Failed to translate '{text}': HTTP {status}",
                status = response.status()
            );
            return Err(TranslationAdapterError::UnexpectedResponse(format!(
                "HTTP {}",
                response.status()
            )));
        }

        // parse response
        debug!("Parsing response for text '{text}'");
        let translate_response: TranslateResponse = response
            .json()
            .await
            .map_err(|err| TranslationAdapterError::ParseError(err.to_string()))?;
        trace!("Parsed translate response: {translate_response:?}");

        // check if success is not zero
        if translate_response.success.total == 0 {
            error!("Translation API returned unsuccessful response for text '{text}'");
            return Err(TranslationAdapterError::UnexpectedResponse(
                "Translation API returned unsuccessful response".to_string(),
            ));
        }

        Ok(translate_response.contents.translated)
    }

    #[inline]
    /// Get the Shakespeare translation URL.
    fn shakespeare_url(&self) -> TranslationAdapterResult<Url> {
        self.endpoint
            .join("shakespeare")
            .map_err(|err| TranslationAdapterError::UnknownError(err.to_string()))
    }

    #[inline]
    /// Get the Yoda translation URL.
    fn yoda_url(&self) -> TranslationAdapterResult<Url> {
        self.endpoint
            .join("yoda")
            .map_err(|err| TranslationAdapterError::UnknownError(err.to_string()))
    }
}

impl TranslationAdapter for FunTranslationsAdapter {
    async fn translate_into_shakespeare(&self, text: &str) -> TranslationAdapterResult<String> {
        self.translate(self.shakespeare_url()?, text).await
    }

    async fn translate_into_yoda(&self, text: &str) -> TranslationAdapterResult<String> {
        self.translate(self.yoda_url()?, text).await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_should_build_default_fun_translations_adapter() {
        let adapter = FunTranslationsAdapter::default();
        assert_eq!(
            adapter.endpoint.as_str(),
            "https://api.funtranslations.com/translate/"
        );
        assert_eq!(adapter.timeout, DEFAULT_API_TIMEOUT);
    }

    #[test]
    fn test_should_build_custom_fun_translations_adapter() {
        let custom_url = Url::parse("https://custom-funtranslations.com/api/").unwrap();
        let custom_timeout = Duration::from_secs(10);
        let adapter = FunTranslationsAdapter::new(custom_url.clone(), custom_timeout);
        assert_eq!(adapter.endpoint, custom_url);
        assert_eq!(adapter.timeout, custom_timeout);
    }

    #[test]
    fn test_should_get_shakespeare_url() {
        let adapter = FunTranslationsAdapter::default();
        let shakespeare_url = adapter.shakespeare_url().unwrap();
        assert_eq!(
            shakespeare_url.as_str(),
            "https://api.funtranslations.com/translate/shakespeare"
        );
    }

    #[test]
    fn test_should_get_yoda_url() {
        let adapter = FunTranslationsAdapter::default();
        let yoda_url = adapter.yoda_url().unwrap();
        assert_eq!(
            yoda_url.as_str(),
            "https://api.funtranslations.com/translate/yoda"
        );
    }

    #[tokio::test]
    #[ignore] // Ignored to avoid hitting the real API during tests
    async fn test_should_translate_into_yoda() {
        let adapter = FunTranslationsAdapter::default();
        let text = "You must unlearn what you have learned.";
        let translated = adapter.translate_into_yoda(text).await;
        assert_eq!(
            translated,
            Ok("Learned,  you must unlearn what you have.".to_string())
        );
    }

    #[tokio::test]
    #[ignore] // Ignored to avoid hitting the real API during tests
    async fn test_should_translate_into_shakespeare() {
        let adapter = FunTranslationsAdapter::default();
        let text = "To be, or not to be, that is the question.";
        let translated = adapter.translate_into_shakespeare(text).await;
        assert_eq!(
            translated,
            Ok("To beest,  or not to beest,  yond is the question.".to_string())
        );
    }
}
