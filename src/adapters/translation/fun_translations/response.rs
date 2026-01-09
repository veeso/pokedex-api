/// The response structure for the Fun Translations API
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateResponse {
    pub success: TranslateSuccess,
    pub contents: TranslateContents,
}

/// The success metadata returned by the Fun Translations API
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateSuccess {
    pub total: u32,
}

/// The translated content returned by the Fun Translations API
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateContents {
    pub translated: String,
}
