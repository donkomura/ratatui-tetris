use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const CELL: char = '□';

fn render_header(f: &mut Frame, app: &App, chunk: Rect) {
    let header_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk);
    let title_block = Paragraph::new(Text::from(vec![Line::from(Span::styled(
        "Tetris",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))]))
    .left_aligned()
    .block(
        Block::default()
            .border_set(symbols::border::PLAIN)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM),
    );
    f.render_widget(title_block, header_area[0]);
    let score_block = Paragraph::new(format!("Score: {}", app.score))
        .right_aligned()
        .block(
            Block::default()
                .border_set(symbols::border::PLAIN)
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM),
        );
    f.render_widget(score_block, header_area[1]);
}

fn render_body(f: &mut Frame, app: &App, chunk: Rect) {
    let body_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10)])
        .split(chunk);
    let mut lines: Vec<Line> = Vec::new();
    for (_, row) in app.board.iter().enumerate() {
        let mut rs = String::new();
        for (_, cell) in row.iter().enumerate() {
            if *cell == 0 {
                rs.push(' ');
            } else {
                rs.push(CELL);
            }
        }
        lines.push(Line::from(rs));
    }
    let text = Text::from(lines);
    let block = Paragraph::new(text).block(Block::bordered());
    f.render_widget(block, body_area[0]);
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(22)])
        .split(f.size());

    render_header(f, app, chunks[0]);
    render_body(f, app, chunks[1]);
}
