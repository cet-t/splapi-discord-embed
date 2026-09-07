mod mode;
mod response;
mod rule;
mod schedule;

use axum::{
    extract::{Path, Query, State},
    response::Html,
};
pub use mode::*;
pub use response::*;
pub use rule::*;
pub use schedule::*;

use crate::{
    data::{Cache, EmbedQuery, ScheduleInput},
    helper::{error_html, render_embed_html},
};

fn build_url(mode: self::Mode, sche: self::Schedule) -> String {
    format!("https://spla3.yuu26.com/api/{mode}/{sche}")
}

async fn enquiry(client: reqwest::Client, url: String) -> anyhow::Result<self::RawResponse> {
    let res = crate::get!(client, url);
    Ok(serde_json::from_str(&res)?)
}

async fn q(
    client: reqwest::Client,
    mode: self::Mode,
    schedule: self::Schedule,
) -> anyhow::Result<self::RawResponse> {
    enquiry(client, build_url(mode, schedule)).await
}

async fn q_after(
    client: reqwest::Client,
    mode: self::Mode,
    sche: self::Schedule,
) -> anyhow::Result<self::RawScheduleInfo> {
    let self::Schedule::After(index) = sche else {
        anyhow::bail!("q_after requires a Schedule::After variant")
    };

    let r = q(client, mode, sche).await?;
    r.results
        .into_iter()
        .nth(index as usize)
        .ok_or_else(|| anyhow::anyhow!("n={index} is out of range"))
}

// --- open ---

pub async fn get_open_schedule(
    State(cache): State<Cache>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<ScheduleInput>,
) -> Html<String> {
    get_info(cache.client, schedule, Mode::BankaraOpen, query)
        .await
        .unwrap_or(error_html())
}

pub async fn get_open_now(
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

pub async fn get_regular_schedule(
    State(cache): State<Cache>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<ScheduleInput>,
) -> Html<String> {
    get_info(cache.client, schedule, Mode::Regular, query)
        .await
        .unwrap_or(error_html())
}

pub async fn get_regular_now(
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
    schedule: ScheduleInput,
    mode: Mode,
    query: EmbedQuery,
) -> anyhow::Result<Html<String>> {
    Ok(Html(match schedule {
        ScheduleInput::Now => {
            let r = q(client, mode, Schedule::Now).await?;
            render_embed_html(r.results.first().ok_or(anyhow::anyhow!("ERROR"))?, query.t)?
        }
        ScheduleInput::Next => {
            let info = q_after(client, mode, Schedule::After(query.n.unwrap_or(1))).await?;
            render_embed_html(&info, query.t)?
        }
    }))
}
