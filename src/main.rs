mod cliargs;
mod helper;
mod splatoon;

use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use clap::Parser;

use crate::{
    cliargs::Cli,
    helper::render_embed_html,
    splatoon::{Mode, Schedule, q, q_after},
};

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct EmbedQuery {
    /// cache buster
    t: Option<u32>,
    /// schedule index
    #[serde(alias = "i")]
    n: Option<u8>,
}

macro_rules! helper {
    ($cb:expr) => {
        ::axum::routing::get(|s, q| async move {
            ::axum::response::Html($cb(s, q).await.unwrap_or_else(|_| "ERROR".to_owned()))
        })
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let port = cli.port()?;
    let addr = format!("0.0.0.0:{port}");

    println!("Server Start: {addr}");

    let client = reqwest::Client::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/open", helper!(get_open_now))
        .route("/open/now", helper!(get_open_now))
        .route("/open/next", helper!(get_open_next))
        .route("/regular", helper!(get_regular_now))
        .route("/regular/now", helper!(get_regular_now))
        .route("/regular/next", helper!(get_regular_next))
        .with_state(client);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_open_now(
    State(client): State<reqwest::Client>,
    Query(EmbedQuery { t, .. }): Query<EmbedQuery>,
) -> anyhow::Result<String> {
    let response = q(&client, Mode::BankaraOpen, Schedule::Now).await?;
    if response.results.is_empty() {
        anyhow::bail!("")
    } else {
        render_embed_html(&response.results[0], t)
    }
}

async fn get_open_next(
    State(client): State<reqwest::Client>,
    Query(EmbedQuery { t, n, .. }): Query<EmbedQuery>,
) -> anyhow::Result<String> {
    get_next(&client, Mode::BankaraOpen, n, t).await
}

async fn get_regular_now(
    State(client): State<reqwest::Client>,
    Query(EmbedQuery { t, .. }): Query<EmbedQuery>,
) -> anyhow::Result<String> {
    let response = q(&client, Mode::Regular, Schedule::Now).await?;
    if response.results.is_empty() {
        anyhow::bail!("")
    }
    render_embed_html(&response.results[0], t)
}

async fn get_regular_next(
    State(client): State<reqwest::Client>,
    Query(EmbedQuery { t, n, .. }): Query<EmbedQuery>,
) -> anyhow::Result<String> {
    get_next(&client, Mode::Regular, n, t).await
}

async fn get_next(
    client: &reqwest::Client,
    mode: Mode,
    n: Option<u8>,
    t: Option<u32>,
) -> anyhow::Result<String> {
    let info = q_after(client, mode, Schedule::After(n.unwrap_or(1))).await?;
    render_embed_html(&info, t)
}
