//! Web server tests

use axum_test::TestServer;

use super::*;
use crate::adapters::pokedex::MockPokedexAdapter;
use crate::adapters::translation::MockTranslationAdapter;
use crate::model::Pokemon;

const DEFAULT_DESCRIPTION: &str = "A yellow electric mouse.";
const SHAKESPEARE_TRANSLATION: &str = "Thee electric mouse.";
const YODA_TRANSLATION: &str = "mouse, a yellow electric.";

#[tokio::test]
async fn test_should_get_pokemon() {
    let server = test_server();

    let response = server.get("/pokemon/pikachu").await;
    response.assert_status_ok();

    // get body
    let pokemon = response.json::<Pokemon>();

    assert_eq!(pokemon.name, "pikachu");
    assert_eq!(pokemon.description, DEFAULT_DESCRIPTION);
    assert_eq!(pokemon.habitat.as_deref(), Some("forest"));
    assert_eq!(pokemon.is_legendary, false);
}

#[tokio::test]
async fn test_should_not_get_unknown_pokemon() {
    let server = test_server_with_pokemon(None);

    let response = server.get("/pokemon/unknown").await;
    response.assert_status_not_found();
}

#[tokio::test]
async fn test_should_get_translated_pokemon_shakespeare() {
    let server = test_server();
    let response = server.get("/pokemon/translated/pikachu").await;
    response.assert_status_ok();
    let pokemon = response.json::<Pokemon>();
    assert_eq!(pokemon.name, "pikachu");
    assert_eq!(pokemon.description, SHAKESPEARE_TRANSLATION);
}

#[tokio::test]
async fn test_should_get_translated_pokemon_yoda_if_legendary() {
    let server = test_server_with_pokemon(Some(Pokemon {
        name: "mewtwo".to_string(),
        description: DEFAULT_DESCRIPTION.to_string(),
        habitat: Some("napoli".to_string()),
        is_legendary: true,
    }));
    let response = server.get("/pokemon/translated/mewtwo").await;
    response.assert_status_ok();
    let pokemon = response.json::<Pokemon>();
    assert_eq!(pokemon.name, "mewtwo");
    assert_eq!(pokemon.description, YODA_TRANSLATION);
}

#[tokio::test]
async fn test_should_get_translated_pokemon_yoda_if_cave() {
    let server = test_server_with_pokemon(Some(Pokemon {
        name: "zubat".to_string(),
        description: DEFAULT_DESCRIPTION.to_string(),
        habitat: Some("cave".to_string()),
        is_legendary: false,
    }));
    let response = server.get("/pokemon/translated/zubat").await;
    response.assert_status_ok();
    let pokemon = response.json::<Pokemon>();
    assert_eq!(pokemon.name, "zubat");
    assert_eq!(pokemon.description, YODA_TRANSLATION);
}

#[tokio::test]
async fn test_should_not_get_translated_pokemon_if_unexisting() {
    let server = test_server_with_pokemon(None);
    let response = server.get("/pokemon/translated/missingno").await;
    response.assert_status_not_found();
}

#[tokio::test]
async fn test_should_run_webserver() {
    let app_data = mock_state(None);
    let web_server = WebServer::new(app_data);
    let addr = "127.0.0.1:0".parse().unwrap();
    let server_handle = tokio::spawn(async move {
        web_server.run(addr).await.unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    server_handle.abort();
}

fn test_server() -> TestServer {
    let router = mock_router(Some(Pokemon {
        name: "pikachu".to_string(),
        description: DEFAULT_DESCRIPTION.to_string(),
        habitat: Some("forest".to_string()),
        is_legendary: false,
    }));
    TestServer::new(router).expect("Failed to create test server")
}

fn test_server_with_pokemon(pokemon: Option<Pokemon>) -> TestServer {
    let router = mock_router(pokemon);
    TestServer::new(router).expect("Failed to create test server")
}

fn mock_router(pokemon: Option<Pokemon>) -> Router {
    let app_state = mock_state(pokemon);
    let app_state = Arc::new(app_state);
    Router::new()
        .route("/pokemon/{name}", axum::routing::get(routes::get_pokemon))
        .route(
            "/pokemon/translated/{name}",
            axum::routing::get(routes::get_translated_pokemon),
        )
        .with_state(app_state)
}

fn mock_state(pokemon: Option<Pokemon>) -> AppState<MockPokedexAdapter, MockTranslationAdapter> {
    AppState {
        pokedex_adapter: pokemon
            .map(MockPokedexAdapter::new)
            .unwrap_or_else(MockPokedexAdapter::not_found),
        translation_adapter: MockTranslationAdapter::new(SHAKESPEARE_TRANSLATION, YODA_TRANSLATION),
    }
}
