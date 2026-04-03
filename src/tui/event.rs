use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::app::App;

const POLL_TIMEOUT: Duration = Duration::from_millis(50);

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(POLL_TIMEOUT)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Right => app.next_tab(),
                    KeyCode::Left => app.prev_tab(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
