use crate::adapters::translation::{TranslationAdapter, TranslationAdapterResult};

/// A mock translation adapter for testing purposes.
pub struct MockTranslationAdapter {
    /// predefined responses for shakespeare translation
    shakespeare_response: String,
    /// predefined responses for yoda translation
    yoda_response: String,
}

impl MockTranslationAdapter {
    /// Creates a new instance of [`MockTranslationAdapter`] with the given predefined responses.
    pub fn new<S>(shakespeare_response: S, yoda_response: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            shakespeare_response: shakespeare_response.into(),
            yoda_response: yoda_response.into(),
        }
    }
}

impl TranslationAdapter for MockTranslationAdapter {
    async fn translate_into_shakespeare(&self, _text: &str) -> TranslationAdapterResult<String> {
        Ok(self.shakespeare_response.clone())
    }

    async fn translate_into_yoda(&self, _text: &str) -> TranslationAdapterResult<String> {
        Ok(self.yoda_response.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_mock_translation_adapter() {
        let mock_adapter = MockTranslationAdapter::new(
            "To be, or not to be, that is the question.",
            "Do or do not, there is no try.",
        );

        let shakespeare_result = mock_adapter
            .translate_into_shakespeare("Hello")
            .await
            .unwrap();
        assert_eq!(
            shakespeare_result,
            "To be, or not to be, that is the question."
        );

        let yoda_result = mock_adapter.translate_into_yoda("Hello").await.unwrap();
        assert_eq!(yoda_result, "Do or do not, there is no try.");
    }
}
