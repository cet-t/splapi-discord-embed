mod cliargs;
mod data;
mod helper;
mod splatoon;

use axum::{Router, routing::get};
use clap::Parser;

use crate::{
    cliargs::Cli,
    data::Cache,
    splatoon::{
        schedule::{get_open_now, get_open_schedule, get_regular_now, get_regular_schedule},
        weapon,
    },
};

macro_rules! helper_now {
    ($cb:ident) => {
        ::axum::routing::get(|c, q| async move {
            ::axum::response::Html(
                $cb(c, q)
                    .await
                    .unwrap_or_else(|_| ::axum::response::Html("Error".to_owned())),
            )
        })
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let addr = format!("0.0.0.0:{}", cli.port()?);

    println!("Server Start: {addr}");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // stage rotation
        .route("/open", helper_now!(get_open_now))
        .route("/regular", helper_now!(get_regular_now))
        .route("/open/{*schedule}", get(get_open_schedule))
        .route("/regular/{*schedule}", get(get_regular_schedule))
        // weapon
        .route("/weapon", get(weapon::get_weapon))
        .with_state(Cache::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
