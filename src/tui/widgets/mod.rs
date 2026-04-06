pub mod chain_picker;
pub mod icon;
pub mod spinner;
pub mod token_search;
pub mod help_overlay;

use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    let horizontal = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0]);
    horizontal[0]
}
