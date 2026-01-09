use serde::{Deserialize, Serialize};

/// A struct representing a Pokémon.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    /// The name of the Pokémon.
    pub name: String,
    /// The pokedex description of the Pokemon.
    pub description: String,
    /// The Pokémon habitat. This field should be defined only for Pokémons between
    /// 1st and 3rd generation, because it was a feature of Pokémon leaf green and
    /// fire red games.
    pub habitat: Option<String>,
    /// Whether the Pokémon is legendary.
    pub is_legendary: bool,
}
