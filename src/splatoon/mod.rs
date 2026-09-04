mod mode;
mod response;
mod rule;
mod schedule;

pub use mode::*;
pub use response::*;
pub use rule::*;
pub use schedule::*;

macro_rules! get {
    ($client:expr, $url:expr) => {
        ::reqwest::Client::get(&$client, &$url)
            .send()
            .await?
            .text()
            .await?
    };
}

fn build_url(mode: self::Mode, sche: self::Schedule) -> String {
    format!("https://spla3.yuu26.com/api/{mode}/{sche}")
}

async fn enquiry(client: reqwest::Client, url: String) -> anyhow::Result<self::RawResponse> {
    let res = get!(client, url);
    Ok(serde_json::from_str(&res)?)
}

pub async fn q(
    client: reqwest::Client,
    mode: self::Mode,
    schedule: self::Schedule,
) -> anyhow::Result<self::RawResponse> {
    enquiry(client, build_url(mode, schedule)).await
}

pub async fn q_after(
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
