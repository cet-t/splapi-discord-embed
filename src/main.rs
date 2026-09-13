mod cliargs;
mod common;
mod data;
mod helper;
mod rgb;
mod splatoon;

use axum::{Router, routing::get};
use clap::Parser;
use tower_http::services::ServeDir;

use crate::{
    cliargs::Cli,
    data::Cache,
    splatoon::{schedule, weapon},
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

    let cache = Cache::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(common::get_url_builder))
        .route("/docs", get(common::get_docs))
        // rotation
        .route("/open", get(schedule::get_open_now))
        .route("/regular", get(schedule::get_regular_now))
        .route("/open/{*schedule}", get(schedule::get_open_schedule))
        .route("/regular/{*schedule}", get(schedule::get_regular_schedule))
        // weapon
        .route("/weapon", get(weapon::get_weapon))
        .with_state(cache)
        .nest_service("/fonts", ServeDir::new("assets/fonts"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
