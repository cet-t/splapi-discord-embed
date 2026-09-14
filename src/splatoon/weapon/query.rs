use axum::extract::{Query, State};
use reqwest::StatusCode;
use urng::{Choice, SplitMix32};

use crate::{
    data::{EmbedQuery, Response},
    helper::{build_html_weapon, error_html},
    state::AppState,
};

pub async fn get_weapon(
    State(AppState { client, cache }): State<AppState>,
    Query(EmbedQuery { n, colour, .. }): Query<EmbedQuery>,
) -> Response {
    let mut cache = cache.lock().await;
    match cache.get_weapons(client).await {
        Some(raw) => {
            let n = n.unwrap_or(1).clamp(1, 8) as usize;
            let mut rng = SplitMix32::default();
            let mut weapons = Vec::with_capacity(n);

            for _ in 0..n {
                let w = rng.choice(raw);
                weapons.push(w);
            }

            (StatusCode::OK, build_html_weapon(&weapons, colour))
        }
        None => error_html(),
    }
}
