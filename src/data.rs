use crate::rgb::Rgb;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbedQuery {
    /// cache buster
    #[serde(rename = "t")]
    pub _t: Option<u32>,
    /// schedule index
    #[serde(alias = "i")]
    pub n: Option<u8>,
    #[serde(alias = "color", alias = "c")]
    pub colour: Option<Rgb>,
}

#[derive(Debug, Clone, Copy, strum::EnumString, serde::Deserialize)]
pub enum ScheduleInput {
    #[strum(serialize = "now")]
    #[serde(alias = "now")]
    Now,
    #[strum(serialize = "next")]
    #[serde(alias = "next")]
    Next,
}

impl std::fmt::Display for ScheduleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScheduleInput::Now => "now",
                ScheduleInput::Next => "next",
            }
        )
    }
}

#[derive(Clone)]
pub struct Cache {
    pub client: reqwest::Client,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
