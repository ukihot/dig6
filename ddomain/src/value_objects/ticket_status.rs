use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum TicketStatus {
    #[default]
    Pending,
    Wip,
    Resolved,
    Canceled,
}
impl From<String> for TicketStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Pending" => TicketStatus::Pending,
            "Wip" => TicketStatus::Wip,
            "Resolved" => TicketStatus::Resolved,
            "Canceled" => TicketStatus::Canceled,
            _ => TicketStatus::Pending,
        }
    }
}

impl From<TicketStatus> for String {
    fn from(status: TicketStatus) -> Self {
        match status {
            TicketStatus::Pending => "Pending".to_string(),
            TicketStatus::Wip => "Wip".to_string(),
            TicketStatus::Resolved => "Resolved".to_string(),
            TicketStatus::Canceled => "Canceled".to_string(),
        }
    }
}
