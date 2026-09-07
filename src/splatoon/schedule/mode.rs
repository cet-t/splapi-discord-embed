#[derive(Debug, Clone, Copy, strum::EnumString)]
pub enum Mode {
    #[strum(to_string = "regular")]
    Regular,
    #[strum(to_string = "bankara-open")]
    BankaraOpen,
    #[strum(to_string = "bankara-challenge")]
    BankaraChallenge,
    #[strum(to_string = "fest")]
    Fest,
    #[strum(to_string = "fest-challenge")]
    FestChallenge,
    #[strum(to_string = "x")]
    X,
    #[strum(to_string = "coop-grouping")]
    CoopGrouping,
    #[strum(to_string = "coop-grouping-team-contest")]
    CoopGroupingTeamContest,
    #[strum(to_string = "event")]
    Event,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Regular => "regular",
                Mode::BankaraOpen => "bankara-open",
                Mode::BankaraChallenge => "bankara-challenge",
                Mode::Fest => "fest",
                Mode::FestChallenge => "fest-challenge",
                Mode::X => "x",
                Mode::CoopGrouping => "coop-grouping",
                Mode::CoopGroupingTeamContest => "coop-grouping-team-contest",
                Mode::Event => "event",
            }
        )
    }
}
