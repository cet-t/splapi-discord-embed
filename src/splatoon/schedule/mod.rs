mod mode;
mod query;
mod response;
mod rule;
mod sche;

use axum::response::Html;

pub use self::mode::*;
pub use self::query::*;
pub use self::response::*;
pub use self::rule::*;
pub use self::sche::*;

use crate::helper::error_text;
use crate::state::AppState;
use crate::{
    data::{EmbedQuery, ScheduleInput},
    helper::build_html_sche,
};

pub fn build_url(mode: self::Mode, sche: self::Schedule) -> String {
    format!("https://spla3.yuu26.com/api/{mode}/{sche}")
}

pub async fn enquiry(client: reqwest::Client, url: String) -> anyhow::Result<self::RawResponse> {
    let res = crate::get!(client, url);
    Ok(serde_json::from_str(&res)?)
}

// --- core ---

async fn get_info(
    AppState { client, cache }: AppState,
    schedule: ScheduleInput,
    mode: Mode,
    query: EmbedQuery,
) -> anyhow::Result<Html<String>> {
    let mut cache = cache.lock().await;
    let info = cache.fetch_schedule(client, mode).await?;

    Ok(match schedule {
        ScheduleInput::Now => build_html_sche(
            info.first().ok_or(anyhow::anyhow!("{}", error_text()))?,
            query,
        ),
        ScheduleInput::Next => {
            let info = info
                .get(query.n.unwrap_or(1) as usize)
                .ok_or(anyhow::anyhow!("{}", error_text()))?;
            build_html_sche(info, query)
        }
    })
}
