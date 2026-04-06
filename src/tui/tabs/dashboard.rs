use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::App;
use crate::tui::state::quote_state::QuoteEntryStatus;
use crate::tui::state::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ])
    .split(area);

    render_left_panel(frame, columns[0], app);
    render_quotes_preview(frame, columns[1], app);
}

fn render_left_panel(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::vertical([
        Constraint::Length(11),
        Constraint::Min(1),
    ])
    .split(area);

    render_logo(frame, rows[0]);
    render_config_summary(frame, rows[1], app);
}

fn render_logo(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::DASHBOARD_BORDER);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let art = crate::tui::widgets::icon::icon_lines();
    let art_height = art.len() as u16;
    let logo = Paragraph::new(art).alignment(Alignment::Center);

    let v = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(art_height),
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

fn render_quotes_preview(frame: &mut Frame, area: Rect, app: &App) {
    let count = app.quote_state.entries.len();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::DASHBOARD_BORDER)
        .title(format!(" Active Quotes ({count}) "));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if count == 0 {
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
        return;
    }

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut lines: Vec<Line> = app
        .quote_state
        .entries
        .iter()
        .take(5)
        .map(|entry| {
            let ttl = entry.expiry as i64 - now_secs as i64;

            let status_icon = match &entry.status {
                QuoteEntryStatus::Active if ttl > 0 => "\u{2713}",
                QuoteEntryStatus::Active => "\u{23f3}",
                QuoteEntryStatus::Expired => "\u{23f3}",
                QuoteEntryStatus::Error(_) => "\u{26a0}",
            };

            let rate_str = match entry.rate {
                Some(r) => format!("{:.4}", r),
                None => "---".to_string(),
            };

            let ttl_str = if ttl > 0 {
                format!("{}s", ttl)
            } else {
                "exp".to_string()
            };

            let style = match &entry.status {
                QuoteEntryStatus::Active if ttl > 0 => theme::DASHBOARD_VALUE,
                _ => theme::DASHBOARD_LABEL,
            };

            Line::from(vec![
                Span::styled(format!("  {status_icon} "), style),
                Span::styled(
                    format!(
                        "{} \u{2192} {:<16}",
                        entry.sell_token.symbol, entry.buy_token.symbol
                    ),
                    style,
                ),
                Span::styled(format!("rate: {rate_str:<12}"), style),
                Span::styled(format!("ttl: {ttl_str}"), style),
            ])
        })
        .collect();

    if count > 5 {
        lines.push(Line::from(Span::styled(
            format!("  ... and {} more", count - 5),
            theme::DASHBOARD_LABEL,
        )));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn truncate_address(addr: &str) -> String {
    if addr.len() > 10 {
        format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
    } else {
        addr.to_string()
    }
}
