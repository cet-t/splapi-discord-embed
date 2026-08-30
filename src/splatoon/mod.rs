mod mode;
mod response;
mod rule;
mod schedule;

pub use mode::*;
pub use response::*;
pub use rule::*;
pub use schedule::*;

pub fn build_url(mode: self::Mode, sche: self::Schedule) -> String {
    format!("https://spla3.yuu26.com/api/{mode}/{sche}")
}

pub async fn enquiry(url: String) -> anyhow::Result<self::RawResponse> {
    let res = reqwest::get(url).await?.text().await?;
    Ok(serde_json::from_str(&res)?)
}

pub async fn q(mode: self::Mode, sche: self::Schedule) -> anyhow::Result<self::RawResponse> {
    enquiry(build_url(mode, sche)).await
}

pub async fn q_after(
    mode: self::Mode,
    sche: self::Schedule,
) -> anyhow::Result<self::RawScheduleInfo> {
    let self::Schedule::After(index) = sche else {
        anyhow::bail!("q_after requires a Schedule::After variant")
    };

    let response = q(mode, sche).await?;
    response
        .results
        .into_iter()
        .nth(index as usize) // n=0 is the currently ongoing match, same as `now`
        .ok_or_else(|| anyhow::anyhow!("n={index} is out of range"))
}
