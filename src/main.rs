mod cliargs;
mod data;
mod helper;
mod splatoon;

use axum::{
    Router,
    extract::{Path, Query, State},
    response::Html,
    routing::get,
};
use clap::Parser;

use crate::{
    cliargs::Cli,
    data::{Cache, EmbedQuery},
    helper::render_embed_html,
    splatoon::{Mode, Schedule, q, q_after},
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

macro_rules! helper_sche {
    ($cb:ident) => {
        ::axum::routing::get(|c, q, s| async move { ::axum::response::Html($cb(c, q, s).await) })
    };
}

fn error_html() -> Html<String> {
    Html("Error".to_owned())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let addr = format!("0.0.0.0:{}", cli.port()?);

    println!("Server Start: {addr}");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/open", helper_now!(get_open_now))
        .route("/regular", helper_now!(get_regular_now))
        .route("/open/{*schedule}", helper_sche!(get_open_schedule))
        .route("/regular/{*schedule}", helper_sche!(get_regular_schedule))
        .with_state(Cache::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// --- open ---

async fn get_open_schedule(
    State(cache): State<Cache>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<data::Schedule>,
) -> Html<String> {
    get_info(cache.client, schedule, Mode::BankaraOpen, query)
        .await
        .unwrap_or(error_html())
}

async fn get_open_now(
    State(cache): State<Cache>,
    Query(query): Query<EmbedQuery>,
) -> anyhow::Result<::axum::response::Html<String>> {
    let r = q(cache.client, Mode::BankaraOpen, Schedule::Now).await?;
    if r.results.is_empty() {
        anyhow::bail!("")
    } else {
        Ok(Html(render_embed_html(
            r.results.first().ok_or(anyhow::anyhow!(""))?,
            query.t,
        )?))
    }
}

// --- regular ---

async fn get_regular_schedule(
    State(cache): State<Cache>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<data::Schedule>,
) -> Html<String> {
    get_info(cache.client, schedule, Mode::Regular, query)
        .await
        .unwrap_or(error_html())
}

async fn get_regular_now(
    State(cache): State<Cache>,
    Query(query): Query<EmbedQuery>,
) -> anyhow::Result<Html<String>> {
    let r = q(cache.client, Mode::Regular, Schedule::Now).await?;
    if r.results.is_empty() {
        anyhow::bail!("")
    } else {
        Ok(Html(render_embed_html(
            r.results.first().ok_or(anyhow::anyhow!(""))?,
            query.t,
        )?))
    }
}

// --- core ---

async fn get_info(
    client: reqwest::Client,
    schedule: data::Schedule,
    mode: Mode,
    query: EmbedQuery,
) -> anyhow::Result<Html<String>> {
    Ok(Html(match schedule {
        data::Schedule::Now => {
            let r = q(client, mode, Schedule::Now).await?;
            render_embed_html(r.results.first().ok_or(anyhow::anyhow!("ERROR"))?, query.t)?
        }
        data::Schedule::Next => {
            let info = q_after(client, mode, Schedule::After(query.n.unwrap_or(1))).await?;
            render_embed_html(&info, query.t)?
        }
    }))
}
