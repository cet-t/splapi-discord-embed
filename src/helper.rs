use axum::response::Html;
use chrono::NaiveDateTime;

use crate::{
    data::EmbedQuery,
    splatoon::{schedule::RawScheduleInfo, weapon::RawWeaponInfo},
};

const SITE_URL: &str = "splat.site";

pub fn error_text() -> String {
    // TODO
    "Error".to_owned()
}

pub fn error_html() -> Html<String> {
    Html(error_text())
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
    let colour = if let Some(c) = query.colour {
        format!("{c}")
    } else {
        info.rule.colour_string()
    };

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

pub fn build_html_weapon(info: &RawWeaponInfo) -> Html<String> {
    let title = escape_html(&info.name.ja_JP);
    let colour = escape_html("#ffffff");

    Html(format!(
        r#"<!DOCTYPE html>
        <html lang="ja">
        <head>
          <meta charset="utf-8">
          <meta property="og:site_name" content="{SITE_URL}">
          <meta property="og:title" content="{title}">
          <meta name="twitter:card" content="summary">
          <meta name="theme-color" content="{colour}">
          <title>{title}</title>
        </head>
        <body>
          <h1>{title}</h1>
        </body>
        </html>"#
    ))
}
