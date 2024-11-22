use crate::dtos::ticket_dto::TicketDTO;
use crate::input_ports::terminal_input_port::TerminalInputPort;
use crate::output_ports::terminal_output_port::TerminalOutputPort;
use color_eyre::Result;
use ddomain::repositories::ticket_repository::TicketRepository;
use ddomain::{entites::ticket::Ticket, value_objects::app_mode::AppMode};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    widgets::TableState,
    Frame,
};

pub struct TerminalInteractor<R: TicketRepository, O: TerminalOutputPort> {
    state: TableState,
    mode: AppMode,
    selected_ticket_index: Option<usize>,
    items: Vec<Ticket>,
    repository: R,
    output_port: O,
}

impl<R: TicketRepository, O: TerminalOutputPort> TerminalInteractor<R, O> {
    pub fn new(repository: R, output_port: O) -> Result<Self> {
        let items = repository.fetch_tickets()?;
        Ok(Self {
            state: TableState::default().with_selected(0),
            mode: AppMode::Normal,
            selected_ticket_index: None,
            items,
            repository,
            output_port,
        })
    }

    fn next_row(&mut self) -> Result<()> {
        let i = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(if i >= self.items.len() - 1 { 0 } else { i + 1 }));
        Ok(())
    }

    fn previous_row(&mut self) -> Result<()> {
        let i = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(if i == 0 { self.items.len() - 1 } else { i - 1 }));
        Ok(())
    }

    fn enter_edit_mode(&mut self) -> Result<()> {
        if let Some(index) = self.state.selected() {
            self.mode = AppMode::Edit;
            self.selected_ticket_index = Some(index);
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(4)]).split(frame.area());

        // DTOに変換
        let ticket_dtos: Vec<TicketDTO> = self
            .items
            .iter()
            .map(|ticket| TicketDTO {
                id: ticket.id.clone(),
                level: ticket.level.clone().into(),
                title: ticket.title.clone(),
                status: ticket.status.clone().into(),
                created_at: ticket.created_at.clone(),
                resolved_at: ticket.resolved_at.clone(),
            })
            .collect();

        match self.mode {
            AppMode::Normal => {
                self.output_port
                    .draw_table(frame, rects[0], self.state.selected(), &ticket_dtos);
                self.output_port.draw_footer(frame, rects[1]);
            }
            AppMode::Edit => {
                let selected_ticket = self
                    .selected_ticket_index
                    .and_then(|i| Some(self.items[i].title.as_str()));
                self.output_port
                    .draw_edit_form(frame, rects[0], selected_ticket);
            }
        }

        Ok(())
    }
}

impl<R: TicketRepository, O: TerminalOutputPort> TerminalInputPort for TerminalInteractor<R, O> {
    fn read_key(&self) -> Result<Option<KeyCode>> {
        if let Event::Key(key_event) = event::read()? {
            Ok(Some(key_event.code))
        } else {
            Ok(None)
        }
    }

    fn next_row(&mut self) -> Result<()> {
        self.next_row()
    }

    fn previous_row(&mut self) -> Result<()> {
        self.previous_row()
    }

    fn enter_edit_mode(&mut self) -> Result<()> {
        self.enter_edit_mode()
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.draw(frame).unwrap();
    }
}
