use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(22)])
        .split(f.size());

    render_header(f, app, chunks[0]);
}
