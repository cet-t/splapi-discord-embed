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

use crate::{
    data::{EmbedQuery, ScheduleInput},
    helper::build_html_sche,
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

// --- core ---

async fn get_info(
    client: reqwest::Client,
    schedule: ScheduleInput,
    mode: Mode,
    query: EmbedQuery,
) -> anyhow::Result<Html<String>> {
    Ok(match schedule {
        ScheduleInput::Now => {
            let r = q(client, mode, Schedule::Now).await?;
            build_html_sche(r.results.first().ok_or(anyhow::anyhow!("ERROR"))?, query)
        }
        ScheduleInput::Next => {
            let info = q_after(client, mode, Schedule::After(query.n.unwrap_or(1))).await?;
            build_html_sche(&info, query)
        }
    })
}
