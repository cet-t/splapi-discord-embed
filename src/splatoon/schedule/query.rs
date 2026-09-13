use axum::{
    extract::{Path, Query, State},
    response::Html,
};

use crate::{
    data::{Cache, EmbedQuery, ScheduleInput},
    helper::error_html,
    splatoon::schedule::{Mode, get_info},
};

pub async fn get_open_schedule(
    State(Cache { client }): State<Cache>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<ScheduleInput>,
) -> Html<String> {
    get_info(client, schedule, Mode::BankaraOpen, query)
        .await
        .unwrap_or(error_html())
}

pub async fn get_open_now(
    State(Cache { client }): State<Cache>,
    Query(query): Query<EmbedQuery>,
) -> Html<String> {
    get_info(client, ScheduleInput::Now, Mode::BankaraOpen, query)
        .await
        .unwrap_or(error_html())
}

pub async fn get_regular_schedule(
    State(Cache { client }): State<Cache>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<ScheduleInput>,
) -> Html<String> {
    get_info(client, schedule, Mode::Regular, query)
        .await
        .unwrap_or(error_html())
}

pub async fn get_regular_now(
    State(Cache { client }): State<Cache>,
    Query(query): Query<EmbedQuery>,
) -> Html<String> {
    get_info(client, ScheduleInput::Now, Mode::Regular, query)
        .await
        .unwrap_or(error_html())
}
