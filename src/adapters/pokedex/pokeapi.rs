use std::time::Duration;

use rand::prelude::IteratorRandom;
use url::Url;

use crate::adapters::pokedex::pokeapi::species::{LANGUAGE_ENGLISH, Species};
use crate::adapters::pokedex::{PokedexAdapter, PokedexAdapterError, PokedexAdapterResult};
use crate::model::Pokemon;

mod species;

const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(30);

/// PokeApiAdapter is an adapter for the PokeAPI service.
#[derive(Clone, Debug)]
pub struct PokeApiAdapter {
    client: reqwest::Client,
    endpoint: Url,
    timeout: Duration,
}

impl Default for PokeApiAdapter {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: Url::parse("https://pokeapi.co/api/v2/").expect("correct URL"),
            timeout: DEFAULT_API_TIMEOUT,
        }
    }
}

impl PokeApiAdapter {
    /// Creates a new [`PokeApiAdapter`] with the given endpoint [`Url`] and timeout.
    pub fn new(endpoint: impl Into<Url>, timeout: Duration) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: endpoint.into(),
            timeout,
        }
    }

    /// Builds the URL for fetching a Pokémon species by name.
    #[inline]
    fn species_endpoint(&self, name: &str) -> PokedexAdapterResult<Url> {
        self.endpoint
            .join("pokemon-species/")
            .and_then(|url| url.join(name))
            .map_err(|err| PokedexAdapterError::ParseError(err.to_string()))
    }

    /// Extracts the [`Pokemon`] data from the given species data.
    fn get_pokemon_data_from_species(
        &self,
        name: &str,
        species: Species,
    ) -> PokedexAdapterResult<Pokemon> {
        let mut rng = rand::rng();
        // get a random english description. If none found, return an error.
        let description = species
            .flavor_text_entries
            .into_iter()
            .filter(|entry| entry.language.name == LANGUAGE_ENGLISH)
            .choose(&mut rng)
            .map(|entry| entry.flavor_text)
            .ok_or_else(|| {
                PokedexAdapterError::UnexpectedResponse("No English description found".to_string())
            })?;
        debug!("Found description: {description}");

        Ok(Pokemon {
            name: name.to_string(),
            description,
            habitat: species.habitat.map(|habitat| habitat.name),
            is_legendary: species.is_legendary,
        })
    }
}

impl PokedexAdapter for PokeApiAdapter {
    async fn fetch_pokemon_by_name(&self, name: &str) -> PokedexAdapterResult<Pokemon> {
        let url = self.species_endpoint(name)?;
        let response = self
            .client
            .get(url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|err| PokedexAdapterError::NetworkError(err.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            error!("Pokémon '{name}' not found");
            return Err(PokedexAdapterError::NotFound);
        }
        if !response.status().is_success() {
            error!(
                "Failed to fetch Pokémon '{name}': HTTP {status}",
                status = response.status()
            );
            return Err(PokedexAdapterError::UnexpectedResponse(format!(
                "HTTP {}",
                response.status()
            )));
        }

        // parse response
        debug!("Parsing response for Pokémon '{name}'");
        let species: Species = response
            .json()
            .await
            .map_err(|err| PokedexAdapterError::ParseError(err.to_string()))?;
        trace!("Parsed species data: {species:?}");

        // extract Pokémon data
        let pokemon = self.get_pokemon_data_from_species(name, species)?;
        debug!("Found pokemon: {pokemon:?}");
        Ok(pokemon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::pokedex::pokeapi::species::{FlavorTextEntry, Language};

    #[test]
    fn test_should_build_default_adapter() {
        let adapter = PokeApiAdapter::default();
        assert_eq!(adapter.endpoint.as_str(), "https://pokeapi.co/api/v2/");
    }

    #[test]
    fn test_should_build_custom_adapter() {
        let custom_url = Url::parse("https://custom-pokeapi.com/api/").unwrap();
        let adapter = PokeApiAdapter::new(custom_url.clone(), Duration::from_secs(10));
        assert_eq!(adapter.endpoint, custom_url);
        assert_eq!(adapter.timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_should_get_pokemon_species_url() {
        let adapter = PokeApiAdapter::default();
        let species_url = adapter.species_endpoint("pikachu").unwrap();
        assert_eq!(
            species_url.as_str(),
            "https://pokeapi.co/api/v2/pokemon-species/pikachu"
        );
    }

    #[test]
    fn test_should_get_pokemon_data_from_species() {
        let adapter = PokeApiAdapter::default();
        let species = Species {
            flavor_text_entries: vec![
                FlavorTextEntry {
                    flavor_text: "A yellow electric mouse.".to_string(),
                    language: Language {
                        name: LANGUAGE_ENGLISH.to_string(),
                    },
                },
                FlavorTextEntry {
                    flavor_text: "Un ratón eléctrico amarillo.".to_string(),
                    language: Language {
                        name: "es".to_string(),
                    },
                },
            ],
            is_legendary: false,
            habitat: Some(species::Habitat {
                name: "forest".to_string(),
            }),
        };
        let pokemon = adapter
            .get_pokemon_data_from_species("pikachu", species)
            .unwrap();
        assert_eq!(pokemon.name, "pikachu");
        assert_eq!(pokemon.description, "A yellow electric mouse.");
        assert_eq!(pokemon.is_legendary, false);
        assert_eq!(pokemon.habitat.as_deref(), Some("forest"));
    }

    #[tokio::test]
    async fn test_should_fetch_pokemon_by_name_with_habitat() {
        let adapter = PokeApiAdapter::default();
        let pokemon = adapter.fetch_pokemon_by_name("lickitung").await.unwrap();
        assert_eq!(pokemon.name, "lickitung");
        assert_eq!(pokemon.habitat.as_deref(), Some("grassland"));
        assert_eq!(pokemon.is_legendary, false);
        assert!(!pokemon.description.is_empty());
    }

    #[tokio::test]
    async fn test_should_fetch_pokemon_by_name_without_habitat() {
        let adapter = PokeApiAdapter::default();
        let pokemon = adapter.fetch_pokemon_by_name("bidoof").await.unwrap();
        assert_eq!(pokemon.name, "bidoof");
        assert_eq!(pokemon.habitat, None);
        assert_eq!(pokemon.is_legendary, false);
        assert!(!pokemon.description.is_empty());
    }

    #[tokio::test]
    async fn test_should_return_error_on_unexisting_pokemon() {
        let adapter = PokeApiAdapter::default();
        let result = adapter
            .fetch_pokemon_by_name("nonexistentpokemon")
            .await
            .unwrap_err();
        assert!(
            matches!(result, PokedexAdapterError::NotFound),
            "expected UnexpectedResponse error, got {result:?}"
        );
    }
}
