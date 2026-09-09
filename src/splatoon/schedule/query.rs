use axum::{
    extract::{Path, Query, State},
    response::Html,
};

use crate::{
    data::{Cache, EmbedQuery, ScheduleInput},
    helper::{error_html, render_embed_html_sche},
    splatoon::schedule::{Mode, Schedule, get_info, q},
};

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
        Ok(render_embed_html_sche(
            r.results.first().ok_or(anyhow::anyhow!(""))?,
            query,
        ))
    }
}

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
        Ok(render_embed_html_sche(
            r.results.first().ok_or(anyhow::anyhow!(""))?,
            query,
        ))
    }
}
