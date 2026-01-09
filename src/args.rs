//! This module defines the command-line arguments for the pokedex-api application.

mod log_level;

use std::path::PathBuf;

pub use self::log_level::LogLevel;

/// pokedex-api is a web server for accessing Pokémon data.
#[derive(argh::FromArgs, Debug)]
pub struct Args {
    /// adapter timeout in seconds [default: 30]
    #[argh(option, default = "30")]
    pub adapter_timeout_secs: u64,
    /// bind address for the web server [default: 127.0.0.1]
    #[argh(option, short = 'b', default = "String::from(\"127.0.0.1\")")]
    pub bind_address: String,
    /// fun translation custom endpoint URL [default: https://api.funtranslations.com/]
    #[argh(option)]
    pub funtranslation_endpoint: Option<String>,
    /// the path to the log file [default: log only to stdout]
    #[argh(option, short = 'L')]
    pub log_file: Option<PathBuf>,
    /// apply log filtering to target matching the given string [default: no filtering]
    #[argh(option)]
    pub log_filter: Option<String>,
    /// the log level [default: info]
    #[argh(option, short = 'l', default = "LogLevel::Info")]
    pub log_level: LogLevel,
    /// pokeapi custom endpoint URL [default: https://pokeapi.co/api/v2/]
    #[argh(option)]
    pub pokeapi_endpoint: Option<String>,
    /// the port the web server will listen on [default: 5000]
    #[argh(option, short = 'p', default = "5000")]
    pub port: u16,
}
