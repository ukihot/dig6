use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{self, Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    DefaultTerminal, Frame,
};
use style::palette::tailwind;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ticket {
    pub id: String,
    pub level: TicketLevel,
    pub title: String,
    pub status: TicketStatus,
    #[serde(default)]
    pub is_editing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tickets {
    pub tickets: Vec<Ticket>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TicketStatus {
    #[default]
    Pending,
    Wip,
    Resolved,
    Canceled,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TicketLevel {
    #[default]
    One = 1,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
}

pub struct TableColors {
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_row_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
}

impl TableColors {
    pub const fn new(color: &tailwind::Palette) -> Self {
        Self {
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
        }
    }
}

pub const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
    "(Shift + →) next color | (Shift + ←) previous color",
];
const ITEM_HEIGHT: usize = 4;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Edit,
}

pub struct App {
    pub state: TableState,
    pub items: Vec<Ticket>,
    pub scroll_state: ratatui::widgets::ScrollbarState,
    pub colors: TableColors,
    pub mode: Mode,
    pub selected_ticket_index: Option<usize>,
}

impl App {
    pub fn new(items: Vec<Ticket>) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ratatui::widgets::ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            items,
            mode: Mode::Normal,
            selected_ticket_index: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == ratatui::crossterm::event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('l') => self.enter_edit_mode(),
                        KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                        KeyCode::Char('s') => self.save_ticket(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn next_row(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(if i >= self.items.len() - 1 { 0 } else { i + 1 }));
        self.scroll_state = self
            .scroll_state
            .position(self.state.selected().unwrap_or(0) * ITEM_HEIGHT);
    }

    fn previous_row(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(if i == 0 { self.items.len() - 1 } else { i - 1 }));
        self.scroll_state = self
            .scroll_state
            .position(self.state.selected().unwrap_or(0) * ITEM_HEIGHT);
    }

    fn enter_edit_mode(&mut self) {
        if let Some(index) = self.state.selected() {
            let ticket = &mut self.items[index];
            ticket.is_editing = true;
            self.mode = Mode::Edit;
            self.selected_ticket_index = Some(index);
        }
    }

    fn save_ticket(&mut self) {
        if let Some(index) = self.selected_ticket_index {
            let ticket = &mut self.items[index];
            ticket.is_editing = false;
            self.mode = Mode::Normal;
            self.selected_ticket_index = None;
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(4)]).split(frame.area());
        match self.mode {
            Mode::Normal => {
                self.render_table(frame, rects[0]);
                self.render_footer(frame, rects[1]);
            }
            Mode::Edit => {
                self.render_edit_form(frame, rects[0]);
            }
        }
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        // ヘッダーのスタイル設定
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);

        // ヘッダー行の定義
        let header = Row::new(
            ["ID", "レベル", "タイトル", "ステータス"]
                .iter()
                .map(|&s| Cell::from(s)),
        )
        .style(header_style)
        .height(1);

        // 行のデータ
        let rows: Vec<Row> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, ticket)| {
                let row_color = if i % 2 == 0 {
                    self.colors.normal_row_color
                } else {
                    self.colors.alt_row_color
                };

                let style = if self.state.selected() == Some(i) {
                    Style::default()
                        .fg(self.colors.selected_row_style_fg)
                        .bg(row_color)
                } else {
                    Style::default().fg(self.colors.row_fg).bg(row_color)
                };

                Row::new([
                    Cell::from(ticket.id.as_str()),
                    Cell::from(format!("{:?}", ticket.level)),
                    Cell::from(ticket.title.as_str()),
                    Cell::from(format!("{:?}", ticket.status)),
                ])
                .style(style)
            })
            .collect();

        // 列幅の設定
        let widths = vec![
            Constraint::Length(10), // ID列の幅
            Constraint::Length(10), // レベル列の幅
            Constraint::Length(30), // タイトル列の幅
            Constraint::Length(15), // ステータス列の幅
        ];

        // Tableウィジェットのレンダリング
        frame.render_widget(
            Table::new(std::iter::once(header).chain(rows), &widths)
                .block(Block::default().borders(Borders::ALL)),
            area,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(INFO_TEXT.join("\n"))
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("操作ガイド")),
            area,
        );
    }

    fn render_edit_form(&self, frame: &mut Frame, area: Rect) {
        let form_text = if let Some(index) = self.selected_ticket_index {
            // 選択されたチケットの情報を取得
            let ticket = &self.items[index];
            format!("[{}] {}\n内容を編集してください。", ticket.id, ticket.title)
        } else {
            "編集モード: チケットが選択されていません。".to_string()
        };

        let paragraph = Paragraph::new(form_text)
            .block(Block::default().borders(Borders::ALL).title("編集画面"));
        frame.render_widget(paragraph, area);
    }
}
