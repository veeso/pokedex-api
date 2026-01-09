use serde::{Deserialize, Serialize};

/// A struct representing a Pokémon.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    /// The name of the Pokémon.
    pub name: String,
    /// The pokedex description of the Pokemon.
    pub description: String,
    /// THe pokemon habitat.
    pub habitat: String,
    /// Whether the Pokémon is legendary.
    pub is_legendary: bool,
}
