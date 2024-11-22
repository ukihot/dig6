pub mod ticket_repository_impl;
use ddomain::entites::ticket::Ticket;
use serde::Deserialize;

// tomlパース用
#[derive(Deserialize, Debug)]
pub struct TicketCollection {
    pub ticket_data: Vec<Ticket>,
}
