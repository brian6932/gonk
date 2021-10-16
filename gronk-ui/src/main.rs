use std::{
    error::Error,
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, Event as CTEvent, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod browser;
mod ui;

enum Event {
    Input(KeyEvent),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(stdout());

    let mut terminal = Terminal::new(backend)?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(100);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CTEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut app = App::new();

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('c') => {
                    if event.modifiers == KeyModifiers::CONTROL {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        break;
                    }
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                KeyCode::Esc => app.on_escape(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
    }

    Ok(())
}
