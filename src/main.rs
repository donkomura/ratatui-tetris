use color_eyre::eyre::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use ratatui::{backend::CrosstermBackend, terminal::Terminal};
use ratatui_tetris::event::{self, EventHandler};
use ratatui_tetris::{app::App, handler::handle_key_events, tui::Tui};
use std::io;
use std::path::PathBuf;

const LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} | {l} | {f}:{L} | {m}{n}";

pub fn initialize_logging() {
    let data_local_dir = if let Ok(s) = std::env::var("RATATUI_TETRIS_DATA") {
        PathBuf::from(s)
    } else {
        dirs::data_local_dir()
            .expect("Unable to find data directory for ratatui-tetris")
            .join("ratatui-tetris")
    };

    std::fs::create_dir_all(&data_local_dir)
        .unwrap_or_else(|_| panic!("Unable to create {:?}", data_local_dir));

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .append(false)
        .build(data_local_dir.join("ratatui-tetris.log"))
        .expect("Failed to build log file appender.");

    let levelfilter = match std::env::var("RATATUI_TETRIS_LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .as_str()
    {
        "off" => LevelFilter::Off,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .logger(Logger::builder().build("ratatui_tetris", levelfilter))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("Failed to build logging config.");

    log4rs::init_config(config).expect("Failed to initialize logging.");
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    initialize_logging();

    let mut app = App::new();
    while app.running {
        tui.draw(&mut app)?;
        app.check_state();

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
