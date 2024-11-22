use crate::value_objects::ticket_level::TicketLevel;
use crate::value_objects::ticket_status::TicketStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Ticket {
    pub id: String,
    pub level: TicketLevel,
    pub title: String,
    pub status: TicketStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl Ticket {
    pub fn new(id: String, level: TicketLevel, title: String, status: TicketStatus) -> Self {
        Ticket {
            id,
            level,
            title,
            status,
            created_at: Utc::now(),
            resolved_at: None,
        }
    }

    pub fn set_status(&mut self, new_status: TicketStatus) {
        if new_status == TicketStatus::Resolved {
            // 解決日時を現在のUTC時間に設定
            self.resolved_at = Some(Utc::now());
        }
        self.status = new_status;
    }
}
