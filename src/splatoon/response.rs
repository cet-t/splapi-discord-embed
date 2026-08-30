#![allow(dead_code)]

use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer};

use crate::splatoon::Rule;

#[derive(Clone, Deserialize)]
pub struct RawResponse {
    pub results: Vec<self::RawScheduleInfo>,
}

fn deserialize_ndt<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDateTime, D::Error> {
    let s = String::deserialize(d)?;
    let dt =
        DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%z").map_err(serde::de::Error::custom)?;
    Ok(dt.naive_local())
}

#[derive(Clone, Deserialize)]
pub struct RawScheduleInfo {
    #[serde(deserialize_with = "deserialize_ndt")]
    pub start_time: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_ndt")]
    pub end_time: NaiveDateTime,
    pub rule: Rule,
    pub stages: Vec<RawStageInfo>,
    pub is_fest: bool,
}

#[derive(Clone, Deserialize)]
pub struct RawStageInfo {
    id: u32,
    pub name: String,
    pub image: String,
}
