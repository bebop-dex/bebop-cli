use ratatui::style::{Color, Modifier, Style};

pub const HEADER_BG: Style = Style::new().bg(Color::DarkGray);
pub const HEADER_TITLE: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
pub const HEADER_CHAIN: Style = Style::new().fg(Color::Cyan);
pub const HEADER_STATUS: Style = Style::new().fg(Color::Green);

pub const TAB_ACTIVE: Style = Style::new()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);
pub const TAB_INACTIVE: Style = Style::new().fg(Color::Gray);

pub const CONTENT_TEXT: Style = Style::new().fg(Color::White);

pub const FOOTER_BG: Style = Style::new().bg(Color::DarkGray);
pub const FOOTER_KEY: Style = Style::new().fg(Color::White);
