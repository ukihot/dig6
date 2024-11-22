use crate::dtos::ticket_dto::TicketDTO;
use ratatui::{layout::Rect, Frame};

pub trait TerminalOutputPort {
    fn draw_table(
        &self,
        frame: &mut Frame,
        area: Rect,
        selected_index: Option<usize>,
        tickets: &Vec<TicketDTO>,
    );
    fn draw_footer(&self, frame: &mut Frame, area: Rect);
    fn draw_edit_form(&self, frame: &mut Frame, area: Rect, selected_ticket: Option<&str>);
}
