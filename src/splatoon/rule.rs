use std::str::FromStr;

#[derive(Debug, Clone, Copy, strum::EnumString)]
pub enum Rule {
    #[strum(to_string = "TURF_WAR")]
    TurfWar,
    #[strum(to_string = "AREA")]
    SplatZones,
    #[strum(to_string = "LOFT")]
    TowerControl,
    #[strum(to_string = "GOAL")]
    Rainmaker,
    #[strum(to_string = "CLAM")]
    ClamBlitz,
}

impl<'de> serde::Deserialize<'de> for Rule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct RawRule {
            key: String,
        }

        let raw = RawRule::deserialize(d)?;
        raw.key.parse().map_err(serde::de::Error::custom)
    }
}

impl Rule {
    pub fn colour_string(&self) -> String {
        match self {
            Rule::TurfWar => "#00ff00",
            _ => "#ffa500",
        }
        .to_owned()
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rule::TurfWar => "ナワバリバトル",
                Rule::SplatZones => "ガチエリア",
                Rule::TowerControl => "ガチヤグラ",
                Rule::Rainmaker => "ガチホコバトル",
                Rule::ClamBlitz => "ガチアサリ",
            }
        )
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for Rule {
    fn into(self) -> String {
        self.to_string()
    }
}
