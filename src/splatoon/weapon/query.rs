use axum::{extract::State, response::Html};
use urng::{Choice, SplitMix32};

use crate::{
    data::Cache,
    helper::{error_html, render_embed_html_weapon},
    splatoon::weapon::q,
};

pub async fn get_weapon(State(cache): State<Cache>) -> Html<String> {
    let Ok(weapons_raw) = q(cache.client).await else {
        return error_html();
    };

    let mut rng = SplitMix32::default();
    let w = rng.choice(&weapons_raw);

    render_embed_html_weapon(w)
}
