use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::{self, Marker},
    text::{Line, Span, Text},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
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

fn render_body(f: &mut Frame, app: &App, chunk: Rect) {
    let body_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(app.width() + 2)])
        .split(chunk);
    let block = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .marker(Marker::HalfBlock)
        .y_bounds([0.0, app.height() as f64])
        .x_bounds([0.0, app.width() as f64])
        .paint(|ctx| {
            let height = app.height() as usize;
            for (i, row) in app.board.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if *cell == 0 {
                        continue;
                    }
                    ctx.draw(&Rectangle {
                        y: height.saturating_sub(i + 1) as f64,
                        x: (j + 1) as f64,
                        width: 0.0,
                        height: 0.0,
                        color: Color::Red,
                    });
                }
            }
        });
    f.render_widget(block, body_area[0]);
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(app.height() + 2)])
        .split(f.size());

    render_header(f, app, chunks[0]);
    render_body(f, app, chunks[1]);
}
