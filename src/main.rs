use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table,
        TableState,
    },
    DefaultTerminal, Frame,
};
use style::palette::tailwind;

const PALETTES: [tailwind::Palette; 4] = [
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    App::new().run(terminal)?;
    ratatui::restore();
    Ok(())
}

#[derive(Debug)]
enum Level {
    One = 1,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
}

#[derive(Debug)]
enum Status {
    Pending,
    Wip,
    Resolved,
    Canceled,
}

#[derive(Debug)]
struct Ticket {
    id: String,
    level: Level,
    title: String,
    status: Status,
}

impl Ticket {
    fn new(id: &str, level: Level, title: &str, status: Status) -> Self {
        Self {
            id: id.into(),
            level,
            title: title.into(),
            status,
        }
    }
}

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

struct App {
    state: TableState,
    items: Vec<Ticket>,
    longest_item_lens: (u16, u16, u16, u16),
    scroll_state: ratatui::widgets::ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    fn new() -> Self {
        let items = vec![
            Ticket::new("001", Level::One, "ユーザー登録エラー", Status::Pending),
            Ticket::new("002", Level::Two, "ログイン不具合", Status::Wip),
            Ticket::new("003", Level::Three, "サーバーダウン", Status::Resolved),
            Ticket::new("004", Level::Five, "API連携不良", Status::Pending),
        ];

        let longest_item_lens =
            items
                .iter()
                .fold((0, 0, 0, 0), |(id, level, title, status), ticket| {
                    (
                        id.max(ticket.id.len() as u16),
                        level.max(format!("{:?}", ticket.level).len() as u16),
                        title.max(ticket.title.len() as u16),
                        status.max(format!("{:?}", ticket.status).len() as u16),
                    )
                });

        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens,
            scroll_state: ratatui::widgets::ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == ratatui::crossterm::event::KeyEventKind::Press {
                    let shift = key.modifiers.contains(KeyModifiers::SHIFT);
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
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

    fn draw(&mut self, frame: &mut Frame) {
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(4)]).split(frame.area());
        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = Row::new(
            ["ID", "レベル", "タイトル", "ステータス"]
                .iter()
                .map(|&s| Cell::from(s)),
        )
        .style(header_style)
        .height(1);

        let rows = self.items.iter().enumerate().map(|(i, ticket)| {
            let row_color = if i % 2 == 0 {
                self.colors.normal_row_color
            } else {
                self.colors.alt_row_color
            };
            Row::new(
                vec![
                    ticket.id.clone(),
                    format!("{:?}", ticket.level),
                    ticket.title.clone(),
                    format!("{:?}", ticket.status),
                ]
                .into_iter()
                .map(|content| Cell::from(Text::from(content))),
            )
            .style(Style::new().fg(self.colors.row_fg).bg(row_color))
            .height(ITEM_HEIGHT.try_into().unwrap())
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2 + 1),
                Constraint::Min(self.longest_item_lens.3),
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("チケット一覧"));

        frame.render_stateful_widget(table, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(Text::from_iter(INFO_TEXT))
                .style(
                    Style::new()
                        .fg(self.colors.row_fg)
                        .bg(self.colors.buffer_bg),
                )
                .centered()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(Style::new().fg(self.colors.footer_border_color)),
                ),
            area,
        );
    }
}
