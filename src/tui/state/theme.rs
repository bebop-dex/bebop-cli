use ratatui::style::{Color, Modifier, Style};

// ── Palette ──────────────────────────────────────────────────────────
const BLACK: Color = Color::Rgb(23, 23, 23);
const GREEN: Color = Color::Rgb(0, 230, 79);
const YELLOW: Color = Color::Rgb(212, 255, 0);
const PURPLE: Color = Color::Rgb(139, 19, 204);
const DARK_BLUE: Color = Color::Rgb(47, 80, 255);
const GREY: Color = Color::Rgb(106, 106, 106);
const ORANGE: Color = Color::Rgb(255, 131, 0);
const RED: Color = Color::Rgb(216, 5, 0);
const LIGHT_PINK: Color = Color::Rgb(255, 140, 255);
const LIGHT_BLUE: Color = Color::Rgb(53, 243, 255);

// ── Header / Footer / Tabs ───────────────────────────────────────────
pub const HEADER_BG: Style = Style::new().bg(BLACK);
pub const HEADER_TITLE: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
pub const HEADER_CHAIN: Style = Style::new().fg(DARK_BLUE);
pub const HEADER_STATUS: Style = Style::new().fg(GREEN);

pub const TAB_ACTIVE: Style = Style::new()
    .fg(GREEN)
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);
pub const TAB_INACTIVE: Style = Style::new().fg(GREY);

pub const CONTENT_TEXT: Style = Style::new().fg(Color::White);

pub const FOOTER_BG: Style = Style::new().bg(BLACK);
pub const FOOTER_KEY: Style = Style::new().fg(Color::White);

// ── Tokens tab ───────────────────────────────────────────────────────
pub const TOKENS_HEADER: Style = Style::new()
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);
pub const TOKENS_ROW: Style = Style::new().fg(Color::White);
pub const TOKENS_SELECTED: Style = Style::new()
    .bg(BLACK)
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);
pub const TOKENS_CHAIN_VALUE: Style = Style::new().fg(DARK_BLUE);
pub const TOKENS_SEARCH_PROMPT: Style = Style::new().fg(YELLOW);
pub const TOKENS_SEARCH_INPUT: Style = Style::new().fg(Color::White);
pub const TOKENS_ADDRESS: Style = Style::new().fg(GREY);
pub const TOKENS_COUNT: Style = Style::new().fg(GREY);
pub const TOKENS_BORDER: Style = Style::new().fg(GREY);

// ── Spinner ──────────────────────────────────────────────────────────
pub const SPINNER: Style = Style::new().fg(LIGHT_PINK);

// ── Chain picker ─────────────────────────────────────────────────────
pub const CHAIN_PICKER_BORDER: Style = Style::new().fg(PURPLE);
pub const CHAIN_PICKER_HIGHLIGHT: Style = Style::new()
    .bg(BLACK)
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);

// ── Dashboard tab ────────────────────────────────────────────────────
pub const DASHBOARD_BORDER: Style = Style::new().fg(GREY);
#[allow(dead_code)]
pub const DASHBOARD_LOGO: Style = Style::new().fg(GREEN).add_modifier(Modifier::BOLD);
pub const DASHBOARD_LABEL: Style = Style::new().fg(GREY);
pub const DASHBOARD_VALUE: Style = Style::new().fg(Color::White);
pub const DASHBOARD_OK: Style = Style::new().fg(GREEN);
pub const DASHBOARD_MISSING: Style = Style::new().fg(ORANGE);

// ── Quote tab — form ─────────────────────────────────────────────────
pub const QUOTE_FORM_LABEL: Style = Style::new().fg(GREY);
pub const QUOTE_FORM_VALUE: Style = Style::new().fg(Color::White);
pub const QUOTE_FORM_ACTIVE: Style = Style::new()
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);
pub const QUOTE_FORM_PLACEHOLDER: Style = Style::new().fg(GREY);
pub const QUOTE_FORM_BORDER: Style = Style::new().fg(GREY);
pub const QUOTE_SUBMIT_READY: Style = Style::new()
    .fg(GREEN)
    .add_modifier(Modifier::BOLD);
pub const QUOTE_SUBMIT_DISABLED: Style = Style::new().fg(GREY);

// ── Quote tab — table ────────────────────────────────────────────────
pub const QUOTE_TABLE_HEADER: Style = Style::new()
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);
pub const QUOTE_TABLE_ROW: Style = Style::new().fg(Color::White);
pub const QUOTE_TABLE_SELECTED: Style = Style::new()
    .bg(BLACK)
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);

pub const QUOTE_TABLE_DIMMED: Style = Style::new().fg(GREY);

// ── Quote tab — price impact ─────────────────────────────────────────
pub const QUOTE_IMPACT_LOW: Style = Style::new().fg(GREEN);
pub const QUOTE_IMPACT_MED: Style = Style::new().fg(ORANGE);
pub const QUOTE_IMPACT_HIGH: Style = Style::new().fg(RED);

// ── Quote tab — TTL ──────────────────────────────────────────────────
pub const QUOTE_TTL_OK: Style = Style::new().fg(GREEN);
pub const QUOTE_TTL_EXPIRED: Style = Style::new().fg(RED);

// ── Quote tab — delete confirmation ──────────────────────────────────
pub const QUOTE_DELETE_PROMPT: Style = Style::new()
    .fg(RED)
    .add_modifier(Modifier::BOLD);

// ── Config tab ───────────────────────────────────────────────────────
pub const CONFIG_LABEL: Style = Style::new().fg(GREY);
pub const CONFIG_VALUE: Style = Style::new().fg(Color::White);
pub const CONFIG_ACTIVE: Style = Style::new()
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);
pub const CONFIG_ERROR: Style = Style::new().fg(RED);

// ── Token search modal ──────────────────────────────────────────────
pub const TOKEN_SEARCH_BORDER: Style = Style::new().fg(PURPLE);
pub const TOKEN_SEARCH_INPUT: Style = Style::new().fg(Color::White);
pub const TOKEN_SEARCH_PROMPT: Style = Style::new().fg(YELLOW);
pub const TOKEN_SEARCH_HIGHLIGHT: Style = Style::new()
    .bg(BLACK)
    .fg(LIGHT_BLUE)
    .add_modifier(Modifier::BOLD);

// ── Help overlay ─────────────────────────────────────────────────────
pub const HELP_BORDER: Style = Style::new().fg(PURPLE);
pub const HELP_TITLE: Style = Style::new().fg(GREEN).add_modifier(Modifier::BOLD);
pub const HELP_SECTION: Style = Style::new().fg(ORANGE).add_modifier(Modifier::BOLD);
pub const HELP_KEY: Style = Style::new().fg(LIGHT_BLUE);
pub const HELP_DESC: Style = Style::new().fg(Color::White);
pub const WARNING_TEXT: Style = Style::new().fg(ORANGE);
