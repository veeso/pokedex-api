#[macro_use]
extern crate tracing;

//mod adapters;
mod args;
mod log;
//mod model;

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

    Ok(())
}
