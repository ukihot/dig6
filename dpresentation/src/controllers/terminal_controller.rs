use color_eyre::Result;
use dapplication::input_ports::terminal_input_port::TerminalInputPort;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};

pub struct TerminalController<T: TerminalInputPort> {
    input_port: T,
}

impl<T: TerminalInputPort> TerminalController<T> {
    pub fn new(input_port: T) -> Self {
        TerminalController { input_port }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.input_port.draw(frame))?;
            if self.handle_event(event::read()?)? {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<bool> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(true), // 終了
                    KeyCode::Char('l') => self.input_port.enter_edit_mode()?,
                    KeyCode::Char('j') | KeyCode::Down => self.input_port.next_row()?,
                    KeyCode::Char('k') | KeyCode::Up => self.input_port.previous_row()?,
                    _ => {}
                }
            }
        }
        Ok(false)
    }
}
