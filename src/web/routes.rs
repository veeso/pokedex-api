use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::adapters::pokedex::{PokedexAdapter, PokedexAdapterError};
use crate::adapters::translation::TranslationAdapter;
use crate::model::Pokemon;
use crate::web::AppState;

/// GET /pokemon/{name} endpoint handler.
pub async fn get_pokemon<P, T>(
    Path(name): Path<String>,
    State(state): State<Arc<AppState<P, T>>>,
) -> Result<Json<Pokemon>, (StatusCode, String)>
where
    P: PokedexAdapter + Send + Sync + 'static,
    T: TranslationAdapter + Send + Sync + 'static,
{
    info!("GET /pokemon/{name}");

    let response = match state.pokedex_adapter.fetch_pokemon_by_name(&name).await {
        Ok(pokemon) => Ok(Json(pokemon)),
        Err(PokedexAdapterError::NotFound) => {
            Err((StatusCode::NOT_FOUND, "Pokemon not found".to_string()))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
    log_response("GET", format!("/pokemon/{name}"), response)
}

/// GET /pokemon/translated/{name} endpoint handler.
pub async fn get_translated_pokemon<P, T>(
    State(state): State<Arc<AppState<P, T>>>,
    Path(name): Path<String>,
) -> Result<Json<Pokemon>, (StatusCode, String)>
where
    P: PokedexAdapter + Send + Sync + 'static,
    T: TranslationAdapter + Send + Sync + 'static,
{
    info!("GET /pokemon/translated/{name}");

    let pokemon = state
        .pokedex_adapter
        .fetch_pokemon_by_name(&name)
        .await
        .map_err(|err| {
            let status_code = match err {
                PokedexAdapterError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            error!("GET /pokemon/translated/{name} {status_code}: {err}");
            (status_code, err.to_string())
        })?;

    // Translate the description based on whether the pokemon is legendary or from a cave habitat
    let translation_response = if pokemon.is_legendary
        || pokemon
            .habitat
            .as_deref()
            .is_some_and(|habitat| habitat == "cave")
    {
        debug!("Translating description of {name} into Yoda style");
        state
            .translation_adapter
            .translate_into_yoda(&pokemon.description)
            .await
    } else {
        debug!("Translating description of {name} into Shakespearean style");
        state
            .translation_adapter
            .translate_into_shakespeare(&pokemon.description)
            .await
    };

    let response = match translation_response {
        Ok(translated_description) => {
            let translated_pokemon = Pokemon {
                description: translated_description,
                ..pokemon
            };
            debug!("Translated description of {name} successfully");
            Ok(Json(translated_pokemon))
        }
        Err(err) => {
            error!("failed to get translation for {name}: {err}");
            // we return the original pokemon data if translation fails
            Ok(Json(pokemon))
        }
    };
    log_response("GET", format!("/pokemon/translated/{name}"), response)
}

/// Utility function which logs the response status of an endpoint and returns it.
fn log_response<T>(
    method: &'static str,
    endpoint: impl std::fmt::Display,
    response: Result<Json<T>, (StatusCode, String)>,
) -> Result<Json<T>, (StatusCode, String)> {
    match &response {
        Ok(_) => info!("{method} {endpoint} 200 OK"),
        Err((status, message)) => error!("{method} {endpoint} {status}: {message}"),
    }
    response
}
