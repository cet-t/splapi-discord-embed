mod filter;
mod query;
mod response;

pub use query::*;
pub use response::*;

const URL: &str = "https://stat.ink/api/v3/weapon";

pub async fn enquiry(client: reqwest::Client) -> anyhow::Result<self::RawResponse> {
    let res = crate::get!(client, URL.to_owned());
    Ok(serde_json::from_str(&res)?)
}
