use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;
use ratatui::Frame;
pub trait TerminalInputPort {
    fn read_key(&self) -> Result<Option<KeyCode>>;
    fn draw(&mut self, frame: &mut Frame);
    fn next_row(&mut self) -> Result<()>;
    fn previous_row(&mut self) -> Result<()>;
    fn enter_edit_mode(&mut self) -> Result<()>;
}
