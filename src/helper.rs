use axum::{http::StatusCode, response::Html};
use chrono::NaiveDateTime;

use crate::{
    data::{EmbedQuery, Response},
    rgb::Rgb,
    splatoon::{schedule::RawScheduleInfo, weapon::RawWeaponInfo},
};

const SITE_URL: &str = "splat.site";

pub fn error_text() -> String {
    // TODO
    "Internal server error".to_owned()
}

pub fn error_html() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Html(error_text()))
}

fn escape_html<S: Into<String>>(s: S) -> String {
    s.into()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn format_dt(dt: NaiveDateTime) -> String {
    dt.format("%m/%d %H:%M").to_string()
}

pub fn build_html_sche(info: &RawScheduleInfo, query: EmbedQuery) -> Html<String> {
    let desc = {
        let stage_names: Vec<_> = info
            .stages
            .iter()
            .map(|s| format!("- {}", s.name.clone()))
            .collect();
        let time = format!(
            "{} - {}",
            format_dt(info.start_time.naive_local()),
            format_dt(info.end_time.naive_local())
        );
        format!("{time}\n{}", stage_names.join("\n"))
    };

    let imgs_meta = {
        let metas: Vec<_> = info
            .stages
            .iter()
            .map(|s| {
                format!(
                    "<meta property=\"og:image\" content=\"{}\">",
                    escape_html(&s.image_url)
                )
            })
            .collect();
        metas.join("\n")
    };

    let imgs_src = {
        let srcs: Vec<_> = info
            .stages
            .iter()
            .map(|s| {
                format!(
                    "<img src=\"{}\" alt=\"stage\" style=\"max-width:100%\">",
                    escape_html(&s.image_url)
                )
            })
            .collect();
        srcs.join("\n")
    };

    let title = escape_html(info.rule);
    let desc = escape_html(&desc);
    let colour = escape_html(query.colour.unwrap_or(info.rule.to_rgb()));

    Html(format!(
        r#"<!DOCTYPE html>
        <html lang="ja">
        <head>
          <meta charset="utf-8">
          <meta property="og:site_name" content="{SITE_URL}">
          <meta property="og:title" content="{title}">
          <meta property="og:description" content="{desc}">
          {imgs_meta}
          <meta name="twitter:card" content="summary">
          <meta name="theme-color" content="{colour}">
          <title>{title}</title>
        </head>
        <body>
          <h1>{title}</h1>
          <p>{desc}</p>
          {imgs_src}
        </body>
        </html>"#
    ))
}

pub fn build_html_weapon(info: &[&RawWeaponInfo], colour: Option<Rgb>) -> Html<String> {
    let title = escape_html("武器抽選");
    let colour = escape_html(colour.unwrap_or(Rgb([0xff, 0xff, 0x10])));
    let weapons: Vec<_> = info.iter().map(|&w| w.name.ja_JP.clone()).collect();

    let w_meta = {
        let weapons: Vec<_> = weapons
            .iter()
            .enumerate()
            .map(|(i, w)| format!("{}. {}", i + 1, escape_html(w)))
            .collect();
        weapons.join("\n")
    };
    let w_body = {
        let weapons: Vec<_> = weapons
            .iter()
            .map(|w| format!("<li>{}</li>", escape_html(w)))
            .collect();
        weapons.join(" ")
    };

    Html(format!(
        r#"<!DOCTYPE html>
        <html lang="ja">
        <head>
          <meta charset="utf-8">
          <meta property="og:site_name" content="{SITE_URL}">
          <meta property="og:title" content="{title}">
          <meta property="og:description" content="{w_meta}">
          <meta name="twitter:card" content="summary">
          <meta name="theme-color" content="{colour}">
          <title>{title}</title>
        </head>
        <body>
          <ul>
            {w_body}
          </ul>
        </body>
        </html>"#
    ))
}
