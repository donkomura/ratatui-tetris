use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, terminal::Terminal};
use ratatui_tetris::event::{self, EventHandler};
use ratatui_tetris::{app::App, tui::Tui};
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let mut app = App::new();
    while !app.should_quit {
        tui.draw(&mut app)?;
        // 落下
        if !app.fall() {
            app.mino.is_falling = false;
        }
        if !app.mino.is_falling {
            // 新規作成
            if !app.spawn() {
                app.should_quit = true;
            }
            app.mino.is_falling = true;
        }

        match tui.events.next()? {
            event::Event::Tick => {}
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.should_quit = true;
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.should_quit = true;
                    }
                }
                KeyCode::Right => {
                    app.move_right();
                }
                KeyCode::Left => {
                    app.move_left();
                }
                KeyCode::Down => {
                    app.move_down();
                }
                KeyCode::Up => {
                    app.rotate();
                }
                _ => {}
            },
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
