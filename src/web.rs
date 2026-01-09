//! This module exposes the web server functionality of the application.

mod routes;
#[cfg(test)]
mod tests;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::adapters::pokedex::PokedexAdapter;
use crate::adapters::translation::TranslationAdapter;

/// Application state holding the adapters for Pokedex and Translation services.
pub struct AppState<P, T>
where
    P: PokedexAdapter + Send + Sync + 'static,
    T: TranslationAdapter + Send + Sync + 'static,
{
    /// the [`PokedexAdapter`] to use
    pub pokedex_adapter: P,
    /// the [`TranslationAdapter`] to use
    pub translation_adapter: T,
}

/// Represents the web server.
pub struct WebServer<P, T>
where
    P: PokedexAdapter + Send + Sync + 'static,
    T: TranslationAdapter + Send + Sync + 'static,
{
    /// Application state
    app_state: AppState<P, T>,
}

impl<P, T> WebServer<P, T>
where
    P: PokedexAdapter + Send + Sync + 'static,
    T: TranslationAdapter + Send + Sync + 'static,
{
    /// Creates a new instance of the web server with the given application state.
    pub fn new(app_state: AppState<P, T>) -> Self {
        Self { app_state }
    }

    /// Runs the web server, listening on the specified socket address.
    pub async fn run(self, listen_to: SocketAddr) -> anyhow::Result<()> {
        info!("Starting web server; binding to {listen_to}...");
        let listener = TcpListener::bind(listen_to).await?;
        debug!("Web server listening on {listen_to}");

        let app_state = Arc::new(self.app_state);
        let router = Router::new()
            .route("/pokemon/{name}", axum::routing::get(routes::get_pokemon))
            .route(
                "/pokemon/translated/{name}",
                axum::routing::get(routes::get_translated_pokemon),
            )
            .with_state(app_state);

        axum::serve(listener, router)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .map_err(|e| anyhow::anyhow!("Web server error: {}", e))
    }

    /// Handles shutdown signals for graceful termination of the web server.
    async fn shutdown_signal() {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C handler");

        info!("Shutting down web server");
    }
}
