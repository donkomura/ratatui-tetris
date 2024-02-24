mod app;
mod event;
mod ui;

use app::App;
use color_eyre::eyre::Result;
use crossterm::{
    event::{KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::EventHandler;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Terminal,
};
use std::io;
use tui::Tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    // create and run app
    let mut app = App::new();
    let res = run_app(&mut app, &mut terminal, 250);

    if let Ok(success) = res {
        if success {
            println!("score: {}", app.score);
        }
    } else if let Err(err) = res {
        println!("Error: {}", err);
    }

    tui.exit()?;
    Ok(())
}

fn run_app<B: Backend>(app: &mut App, terminal: &mut Terminal<B>, tick: u64) -> Result<bool> {
    let events = EventHandler::new(tick);
    while !app.should_quit {
        terminal.draw(|f| ui::ui(f, app))?;
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

        match events.next()? {
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
    Ok(true)
}
