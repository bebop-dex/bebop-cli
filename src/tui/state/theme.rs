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

// Tokens tab
pub const TOKENS_HEADER: Style = Style::new()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);
pub const TOKENS_ROW: Style = Style::new().fg(Color::White);
pub const TOKENS_SELECTED: Style = Style::new()
    .bg(Color::DarkGray)
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);
pub const TOKENS_CHAIN_VALUE: Style = Style::new().fg(Color::Cyan);
pub const TOKENS_SEARCH_PROMPT: Style = Style::new().fg(Color::Yellow);
pub const TOKENS_SEARCH_INPUT: Style = Style::new().fg(Color::White);
pub const TOKENS_ADDRESS: Style = Style::new().fg(Color::DarkGray);
pub const TOKENS_COUNT: Style = Style::new().fg(Color::DarkGray);
pub const TOKENS_BORDER: Style = Style::new().fg(Color::DarkGray);

// Spinner
pub const SPINNER: Style = Style::new().fg(Color::Cyan);

// Chain picker
pub const CHAIN_PICKER_BORDER: Style = Style::new().fg(Color::Cyan);
pub const CHAIN_PICKER_HIGHLIGHT: Style = Style::new()
    .bg(Color::DarkGray)
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);

// Dashboard tab
pub const DASHBOARD_BORDER: Style = Style::new().fg(Color::DarkGray);
pub const DASHBOARD_LOGO: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
pub const DASHBOARD_LABEL: Style = Style::new().fg(Color::Gray);
pub const DASHBOARD_VALUE: Style = Style::new().fg(Color::White);
pub const DASHBOARD_OK: Style = Style::new().fg(Color::Green);
pub const DASHBOARD_MISSING: Style = Style::new().fg(Color::Yellow);

// Quote tab — form
pub const QUOTE_FORM_LABEL: Style = Style::new().fg(Color::Gray);
pub const QUOTE_FORM_VALUE: Style = Style::new().fg(Color::White);
pub const QUOTE_FORM_ACTIVE: Style = Style::new()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);
pub const QUOTE_FORM_PLACEHOLDER: Style = Style::new().fg(Color::DarkGray);
pub const QUOTE_FORM_BORDER: Style = Style::new().fg(Color::DarkGray);
pub const QUOTE_SUBMIT_READY: Style = Style::new()
    .fg(Color::Green)
    .add_modifier(Modifier::BOLD);
pub const QUOTE_SUBMIT_DISABLED: Style = Style::new().fg(Color::DarkGray);

// Quote tab — table
pub const QUOTE_TABLE_HEADER: Style = Style::new()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);
pub const QUOTE_TABLE_ROW: Style = Style::new().fg(Color::White);
pub const QUOTE_TABLE_SELECTED: Style = Style::new()
    .bg(Color::DarkGray)
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);

// Quote tab — price impact
pub const QUOTE_IMPACT_LOW: Style = Style::new().fg(Color::Green);
pub const QUOTE_IMPACT_MED: Style = Style::new().fg(Color::Yellow);
pub const QUOTE_IMPACT_HIGH: Style = Style::new().fg(Color::Red);

// Quote tab — TTL
pub const QUOTE_TTL_OK: Style = Style::new().fg(Color::Green);
pub const QUOTE_TTL_EXPIRED: Style = Style::new().fg(Color::Red);

// Quote tab — delete confirmation
pub const QUOTE_DELETE_PROMPT: Style = Style::new()
    .fg(Color::Red)
    .add_modifier(Modifier::BOLD);

// Token search modal
pub const TOKEN_SEARCH_BORDER: Style = Style::new().fg(Color::Cyan);
pub const TOKEN_SEARCH_INPUT: Style = Style::new().fg(Color::White);
pub const TOKEN_SEARCH_PROMPT: Style = Style::new().fg(Color::Yellow);
pub const TOKEN_SEARCH_HIGHLIGHT: Style = Style::new()
    .bg(Color::DarkGray)
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);
