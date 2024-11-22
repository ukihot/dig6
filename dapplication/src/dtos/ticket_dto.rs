use chrono::{DateTime, Utc};
use ddomain::entites::ticket::Ticket;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TicketDTO {
    pub id: String,
    pub level: String,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl From<TicketDTO> for Ticket {
    fn from(dto: TicketDTO) -> Self {
        Ticket {
            id: dto.id,
            level: dto.level.into(),
            title: dto.title,
            status: dto.status.into(),
            created_at: Utc::now(),
            resolved_at: None,
        }
    }
}
