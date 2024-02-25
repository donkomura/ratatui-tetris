use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, terminal::Terminal};
use ratatui_tetris::event::{self, EventHandler};
use ratatui_tetris::{app::App, handler::handle_key_events, tui::Tui};
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let mut app = App::new();
    while app.running {
        tui.draw(&mut app)?;
        // 落下
        if !app.fall() {
            app.mino.is_falling = false;
        }
        if !app.mino.is_falling {
            // 新規作成
            if !app.spawn() {
                app.quiet();
            }
            app.mino.is_falling = true;
        }

        match tui.events.next()? {
            event::Event::Tick => {}
            event::Event::Key(key_event) => handle_key_events(key_event, &mut app),
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
