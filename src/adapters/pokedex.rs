//! This module provides the API client adapter for interacting with an external API
//! to fetch information for a given Pokémon name.

#[cfg(test)]
mod mock;
mod pokeapi;

#[cfg(test)]
pub use self::mock::MockPokedexAdapter;
pub use self::pokeapi::PokeApiAdapter;
use crate::model::Pokemon;

/// The result type for PokedexAdapter operations
pub type PokedexAdapterResult<T> = Result<T, PokedexAdapterError>;

/// Errors that can occur while interacting with the PokedexAdapter
#[derive(Debug, thiserror::Error)]
pub enum PokedexAdapterError {
    #[error("Pokémon not found")]
    NotFound,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unexpected response from the API: {0}")]
    UnexpectedResponse(String),
}

/// The PokedexAdapter trait defines the interface for fetching Pokémon data
/// from an external API.
pub trait PokedexAdapter: Send + Sync {
    /// fetch a [`Pokemon`] by its name from the external API
    fn fetch_pokemon_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = PokedexAdapterResult<Pokemon>> + Send;
}
