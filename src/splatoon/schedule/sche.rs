#[derive(Debug, Clone, Copy, strum::EnumString)]
pub enum Schedule {
    #[strum(to_string = "now")]
    Now,
    #[strum(to_string = "next")]
    Next,
    #[strum(to_string = "schedule")]
    After,
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                self::Schedule::Now => "now",
                self::Schedule::Next => "next",
                self::Schedule::After => "schedule",
            }
        )
    }
}
