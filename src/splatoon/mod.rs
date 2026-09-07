pub mod schedule;
pub mod weapon;

#[macro_export]
macro_rules! get {
    ($client:expr, $url:expr) => {
        ::reqwest::Client::get(&$client, &$url)
            .send()
            .await?
            .text()
            .await?
    };
}
