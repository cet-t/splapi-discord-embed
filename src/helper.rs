use chrono::NaiveDateTime;

use crate::splatoon::RawScheduleInfo;

const SITE_URL: &str = "splapi3.cet.run";

pub fn render_embed_html(info: &RawScheduleInfo, t: Option<u32>) -> anyhow::Result<String> {
    Ok(build_html(info, t))
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn format_dt(dt: NaiveDateTime) -> String {
    dt.format("%m/%d %H:%M").to_string()
}

fn build_html(info: &RawScheduleInfo, _t: Option<u32>) -> String {
    let desc = {
        let stage_names: Vec<_> = info
            .stages
            .iter()
            .map(|s| format!("- {}", s.name.clone()))
            .collect();
        let time = format!(
            "{} - {}",
            format_dt(info.start_time),
            format_dt(info.end_time)
        );
        format!("{time}\n{}", stage_names.join("\n"))
    };

    let imgs_meta = {
        let metas: Vec<_> = info
            .stages
            .iter()
            .map(|s| format!("<meta property=\"og:image\" content=\"{}\">", s.image))
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
                    s.image
                )
            })
            .collect();
        srcs.join("\n")
    };

    let title = escape_html(&info.rule.to_string());
    let desc = escape_html(&desc);
    let colour = info.rule.colour_string();

    format!(
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
    )
}
