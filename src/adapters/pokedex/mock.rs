use crate::adapters::pokedex::{PokedexAdapter, PokedexAdapterResult};
use crate::model::Pokemon;

/// Mock implementation of the PokedexAdapter for testing purposes.
///
/// It simulates fetching Pokémon data without making actual network requests.
/// It always returns the data for a predefined [`Pokemon`].
pub struct MockPokedexAdapter(Pokemon);

impl From<Pokemon> for MockPokedexAdapter {
    fn from(pokemon: Pokemon) -> Self {
        MockPokedexAdapter(pokemon)
    }
}

impl MockPokedexAdapter {
    /// Creates a new instance of the [`MockPokedexAdapter`] with the given [`Pokemon`] data.
    pub fn new(pokemon: Pokemon) -> Self {
        pokemon.into()
    }
}

impl PokedexAdapter for MockPokedexAdapter {
    async fn fetch_pokemon_by_name(&self, _name: &str) -> PokedexAdapterResult<Pokemon> {
        Ok(self.0.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_should_construct_mock_pokedex_adapter() {
        let mocked_pokemon = Pokemon {
            name: "Pikachu".to_string(),
            description: "An Electric-type Pokémon".to_string(),
            habitat: Some("Emerald Forest".to_string()),
            is_legendary: false,
        };

        let adapter = MockPokedexAdapter::new(mocked_pokemon.clone());

        assert_eq!(adapter.0, mocked_pokemon);
    }

    #[test]
    fn test_should_convert_pokemon_into_mock_adapter() {
        let mocked_pokemon = Pokemon {
            name: "Pikachu".to_string(),
            description: "An Electric-type Pokémon".to_string(),
            habitat: Some("Emerald Forest".to_string()),
            is_legendary: false,
        };

        let adapter: MockPokedexAdapter = mocked_pokemon.clone().into();

        assert_eq!(adapter.0, mocked_pokemon);
    }

    #[tokio::test]
    async fn test_should_fetch_mocked_pokemon_by_name() {
        let mocked_pokemon = Pokemon {
            name: "Pikachu".to_string(),
            description: "An Electric-type Pokémon".to_string(),
            habitat: Some("Emerald Forest".to_string()),
            is_legendary: false,
        };

        let adapter = MockPokedexAdapter::new(mocked_pokemon.clone());

        let fetched_pokemon = adapter.fetch_pokemon_by_name("Pikachu").await.unwrap();

        assert_eq!(fetched_pokemon, mocked_pokemon);
    }
}
