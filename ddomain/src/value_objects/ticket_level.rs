use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum TicketLevel {
    #[default]
    One = 1,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
}

impl From<String> for TicketLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "One" => TicketLevel::One,
            "Two" => TicketLevel::Two,
            "Three" => TicketLevel::Three,
            "Five" => TicketLevel::Five,
            "Eight" => TicketLevel::Eight,
            "Thirteen" => TicketLevel::Thirteen,
            _ => TicketLevel::One,
        }
    }
}

impl From<TicketLevel> for String {
    fn from(level: TicketLevel) -> Self {
        match level {
            TicketLevel::One => "One".to_string(),
            TicketLevel::Two => "Two".to_string(),
            TicketLevel::Three => "Three".to_string(),
            TicketLevel::Five => "Five".to_string(),
            TicketLevel::Eight => "Eight".to_string(),
            TicketLevel::Thirteen => "Thirteen".to_string(),
        }
    }
}
