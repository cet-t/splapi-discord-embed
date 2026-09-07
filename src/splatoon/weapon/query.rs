use axum::{extract::State, response::Html};

use crate::{data::Cache, helper::error_html, splatoon::weapon::q};

pub async fn get_weapon(State(cache): State<Cache>) -> Html<String> {
    let Ok(weapons_raw) = q(cache.client).await else {
        println!("get_weapon failed");
        return error_html();
    };

    Html(format!("{}", weapons_raw.len()))
}
