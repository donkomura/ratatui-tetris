mod app;
mod event;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::EventHandler;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Terminal,
};
use std::{
    io::{self, Write},
    panic,
};

fn reset(mut stream: Box<dyn Write>) -> Result<()> {
    disable_raw_mode()?;
    execute!(stream, LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // panic hook
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset(Box::new(io::stderr())).expect("failed to reset the terminal");
        panic_hook(panic);
    }));

    // create and run app
    let mut app = App::new();
    let res = run_app(&mut app, &mut terminal, 250);
    reset(Box::new(io::stdout()))?; // Use the new instance of stdout
    terminal.show_cursor()?;

    if let Ok(success) = res {
        if success {
            println!("score: {}", app.score);
        }
    } else if let Err(err) = res {
        println!("Error: {}", err);
    }

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
                _ => {}
            },
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        };
    }
    Ok(true)
}
