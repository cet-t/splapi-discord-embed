#![allow(dead_code)]

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer};

use crate::splatoon::Rule;

#[derive(Clone, Deserialize)]
pub struct RawResponse {
    pub results: Vec<self::RawScheduleInfo>,
}

fn deserialize_dt<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<FixedOffset>, D::Error> {
    let s = String::deserialize(d)?;
    let dt =
        DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%z").map_err(serde::de::Error::custom)?;
    Ok(dt)
}

#[derive(Clone, Deserialize)]
pub struct RawScheduleInfo {
    #[serde(deserialize_with = "deserialize_dt")]
    pub start_time: DateTime<FixedOffset>,
    #[serde(deserialize_with = "deserialize_dt")]
    pub end_time: DateTime<FixedOffset>,
    pub rule: Rule,
    pub stages: Vec<RawStageInfo>,
    pub is_fest: bool,
}

#[derive(Clone, Deserialize)]
pub struct RawStageInfo {
    id: u32,
    pub name: String,
    #[serde(rename = "image")]
    pub image_url: String,
}
