use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::App;
use crate::tui::state::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ])
    .split(area);

    render_left_panel(frame, columns[0], app);
    render_quotes_preview(frame, columns[1]);
}

fn render_left_panel(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::vertical([
        Constraint::Length(7),
        Constraint::Min(1),
    ])
    .split(area);

    render_logo(frame, rows[0]);
    render_config_summary(frame, rows[1], app);
}

fn render_logo(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::DASHBOARD_BORDER)
        .title(" Logo ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let logo = Paragraph::new("BEBOP")
        .style(theme::DASHBOARD_LOGO)
        .alignment(Alignment::Center);

    let v = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .split(inner);
    frame.render_widget(logo, v[1]);
}

fn render_config_summary(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::DASHBOARD_BORDER)
        .title(" Config ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chain = app.config.chain.as_deref().unwrap_or("not set");

    let wallet = match &app.config.wallet_address {
        Some(addr) => truncate_address(addr),
        None => "not set".into(),
    };
    let wallet_style = if app.config.wallet_address.is_some() {
        theme::DASHBOARD_VALUE
    } else {
        theme::DASHBOARD_MISSING
    };

    let api_status = if app.config.api_key.is_some() {
        Span::styled("configured \u{2713}", theme::DASHBOARD_OK)
    } else {
        Span::styled("missing \u{2717}", theme::DASHBOARD_MISSING)
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Chain:   ", theme::DASHBOARD_LABEL),
            Span::styled(chain, theme::DASHBOARD_VALUE),
        ]),
        Line::from(vec![
            Span::styled("  Wallet:  ", theme::DASHBOARD_LABEL),
            Span::styled(wallet, wallet_style),
        ]),
        Line::from(vec![
            Span::styled("  API Key: ", theme::DASHBOARD_LABEL),
            api_status,
        ]),
    ];

    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(inner);

    frame.render_widget(Paragraph::new(lines), rows[1]);
}

fn render_quotes_preview(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::DASHBOARD_BORDER)
        .title(" Active Quotes ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let placeholder = Paragraph::new("No active quotes")
        .style(theme::CONTENT_TEXT)
        .alignment(Alignment::Center);

    let v = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .split(inner);
    frame.render_widget(placeholder, v[1]);
}

fn truncate_address(addr: &str) -> String {
    if addr.len() > 10 {
        format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
    } else {
        addr.to_string()
    }
}
