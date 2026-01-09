#[macro_use]
extern crate tracing;

use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use url::Url;

use crate::adapters::pokedex::PokeApiAdapter;
use crate::adapters::translation::FunTranslationsAdapter;
use crate::web::AppState;

mod adapters;
mod args;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse args
    let args: args::Args = argh::from_env();

    // initialize logging
    log::init_log(
        args.log_level.into(),
        args.log_filter,
        args.log_file.as_deref(),
    );

    info!(
        "Starting {app_name} v{app_version}",
        app_name = env!("CARGO_PKG_NAME"),
        app_version = env!("CARGO_PKG_VERSION")
    );

    // init the adapters
    let adapters_timeout = Duration::from_secs(args.adapter_timeout_secs);
    debug!("Initializing adapters with timeout {adapters_timeout:?}...");
    let pokedex_adapter = args
        .pokeapi_endpoint
        .as_ref()
        .map(|endpoint| {
            PokeApiAdapter::new(
                Url::parse(endpoint).expect("invalid endpoint url"),
                adapters_timeout,
            )
        })
        .unwrap_or_default();
    let translation_adapter = args
        .funtranslation_endpoint
        .as_ref()
        .map(|endpoint| {
            FunTranslationsAdapter::new(
                Url::parse(endpoint).expect("invalid endpoint url"),
                adapters_timeout,
            )
        })
        .unwrap_or_default();

    // make the socket address to listen to and start the web server
    let listen_to = SocketAddr::from_str(&format!(
        "{addr}:{port}",
        addr = args.bind_address,
        port = args.port
    ))?;

    // run web server
    debug!("About to start web server...");
    let app_state = AppState {
        pokedex_adapter,
        translation_adapter,
    };
    web::WebServer::new(app_state).run(listen_to).await?;

    Ok(())
}
