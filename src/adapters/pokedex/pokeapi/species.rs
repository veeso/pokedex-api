pub const LANGUAGE_ENGLISH: &str = "en";

/// This struct represents a Pokémon species returned by the `api/v2/pokemon-species/:pokemon` endpoint.
/// here only fields relevant to the application should be defined.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Species {
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub habitat: Option<Habitat>,
    pub is_legendary: bool,
}

/// This struct represents a flavor text entry for a Pokémon species.
///
/// It is used to get the Pokémon description in different languages and versions.
#[derive(Debug, serde::Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
}

/// This struct represents the habitat of a Pokémon species.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Habitat {
    pub name: String,
}

/// This struct represents a language used in the Pokémon species data.
#[derive(Debug, serde::Deserialize)]
pub struct Language {
    pub name: String,
}
