mod app;
mod ui;

use app::App;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Terminal,
};
use std::{
    error::Error,
    io::{self, Write},
    panic,
};

fn reset(mut stream: Box<dyn Write>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stream, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
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
    let res = run_app(&mut app, &mut terminal);
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

fn run_app<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        // 新しいミノの生成
        if !app.mino.is_falling {
            app.mino.is_falling = true;
            if !app.spawn() {
                app.should_quit = true;
            }
        }

        // TODO: move to the other thread
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                _ => {}
            }
        }

        if app.should_quit {
            return Ok(true);
        }
    }
}
