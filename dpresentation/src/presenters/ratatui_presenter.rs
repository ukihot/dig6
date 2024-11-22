use dapplication::{
    dtos::ticket_dto::TicketDTO, output_ports::terminal_output_port::TerminalOutputPort,
};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub struct RatatuiPresenter;

impl RatatuiPresenter {
    pub fn new() -> Self {
        RatatuiPresenter
    }
}

impl TerminalOutputPort for RatatuiPresenter {
    fn draw_table(
        &self,
        frame: &mut Frame,
        area: Rect,
        selected_index: Option<usize>,
        tickets: &Vec<TicketDTO>,
    ) {
        let header_style = Style::default().fg(Color::White).bg(Color::Blue);
        let header = Row::new(
            [
                "ID",
                "Level",
                "Title",
                "Status",
                "Created At",
                "Resolved At",
            ]
            .iter()
            .map(|&s| Cell::from(s)),
        )
        .style(header_style)
        .height(1);

        let rows: Vec<Row> = tickets
            .iter()
            .enumerate()
            .map(|(i, ticket)| {
                let row_style = if selected_index == Some(i) {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White).bg(Color::DarkGray)
                };

                Row::new([
                    Cell::from(ticket.id.as_str()),
                    Cell::from(ticket.level.as_str()),
                    Cell::from(ticket.title.as_str()),
                    Cell::from(ticket.status.as_str()),
                    Cell::from(ticket.created_at.to_rfc3339()), // Created At
                    Cell::from(
                        ticket.resolved_at
                        .map(|dt| dt.to_rfc3339()) // Resolved AtがSomeなら日付を表示
                        .unwrap_or_else(|| "".to_string()), // Noneなら空文字列
                    ),
                ])
                .style(row_style)
            })
            .collect();

        let widths = vec![
            Constraint::Length(10), // Width of ID column
            Constraint::Length(10), // Width of Level column
            Constraint::Length(30), // Width of Title column
            Constraint::Length(15), // Width of Status column
            Constraint::Length(25), // Width of Created At column
            Constraint::Length(25), // Width of Resolved At column
        ];

        frame.render_widget(
            Table::new(std::iter::once(header).chain(rows), &widths)
                .block(Block::default().borders(Borders::ALL).title("Ticket List")),
            area,
        );
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_text = "(q) Exit | (k) Up | (j) Down | (l) Edit Mode";
        frame.render_widget(
            Paragraph::new(footer_text)
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Control Guide"),
                ),
            area,
        );
    }

    fn draw_edit_form(&self, frame: &mut Frame, area: Rect, selected_ticket: Option<&str>) {
        let form_text = match selected_ticket {
            Some(ticket) => format!("Selected Ticket: {}", ticket),
            None => "Edit Mode: No ticket selected.".to_string(),
        };

        let paragraph = Paragraph::new(form_text)
            .block(Block::default().borders(Borders::ALL).title("Edit Screen"));
        frame.render_widget(paragraph, area);
    }
}
