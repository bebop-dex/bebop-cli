use ratatui::text::Span;

use crate::tui::state::theme;

const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn spinner(tick: usize) -> Span<'static> {
    let frame = FRAMES[tick % FRAMES.len()];
    Span::styled(format!("{frame} Loading..."), theme::SPINNER)
}
