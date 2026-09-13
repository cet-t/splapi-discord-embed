use axum::{
    extract::{Query, State},
    response::Html,
};
use urng::{Choice, SplitMix32};

use crate::{
    data::{Cache, EmbedQuery},
    helper::{build_html_weapon, error_html},
    splatoon::weapon::q,
};

pub async fn get_weapon(
    State(Cache { client }): State<Cache>,
    Query(query): Query<EmbedQuery>,
) -> Html<String> {
    match q(client).await {
        Ok(raw) => {
            let mut rng = SplitMix32::default();
            let w = rng.choice(&raw);
            build_html_weapon(w)
        }
        Err(_) => error_html(),
    }
}
