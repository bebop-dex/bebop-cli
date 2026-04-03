use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs};

use super::app::{ActiveTab, App};
use super::state::theme;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    render_header(frame, chunks[0], app);
    render_tab_bar(frame, chunks[1], app.active_tab);
    render_content(frame, chunks[2], app);
    render_footer(frame, chunks[3]);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let version = env!("CARGO_PKG_VERSION");
    let chain = app.config.chain.as_deref().unwrap_or("no chain");

    let wallet_indicator = if app.config.wallet_address.is_some() {
        Span::styled("wallet \u{2713}", theme::HEADER_STATUS)
    } else {
        Span::styled("wallet \u{2717}", theme::HEADER_CHAIN)
    };

    let api_indicator = if app.config.api_key.is_some() {
        Span::styled("api \u{2713}", theme::HEADER_STATUS)
    } else {
        Span::styled("api \u{2717}", theme::HEADER_CHAIN)
    };

    let header = Line::from(vec![
        Span::styled(format!(" bebop-cli v{version}"), theme::HEADER_TITLE),
        Span::raw("    "),
        Span::styled(format!("[{chain} \u{25be}]"), theme::HEADER_CHAIN),
        Span::raw("    "),
        wallet_indicator,
        Span::raw("  "),
        api_indicator,
    ]);

    frame.render_widget(Paragraph::new(header).style(theme::HEADER_BG), area);
}

fn render_tab_bar(frame: &mut Frame, area: Rect, active: ActiveTab) {
    let titles: Vec<Line> = ActiveTab::ALL.iter().map(|t| Line::from(t.title())).collect();

    let tabs = Tabs::new(titles)
        .style(theme::TAB_INACTIVE)
        .highlight_style(theme::TAB_ACTIVE)
        .select(active as usize)
        .divider(" ");

    frame.render_widget(tabs, area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    match app.active_tab {
        ActiveTab::Dashboard => {
            super::tabs::dashboard::render(frame, area, app);
        }
        ActiveTab::Tokens => {
            let content = Paragraph::new("Tokens \u{2014} token browser coming soon")
                .style(theme::CONTENT_TEXT);
            frame.render_widget(content, area);
        }
        ActiveTab::Quote => {
            let content = Paragraph::new("Quote \u{2014} RFQ interface coming soon")
                .style(theme::CONTENT_TEXT);
            frame.render_widget(content, area);
        }
        ActiveTab::Config => {
            let content = Paragraph::new("Config \u{2014} settings editor coming soon")
                .style(theme::CONTENT_TEXT);
            frame.render_widget(content, area);
        }
    }
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Line::from(vec![
        Span::styled(" \u{2190}\u{2192} tabs", theme::FOOTER_KEY),
        Span::raw("  "),
        Span::styled("\u{2191}\u{2193} navigate", theme::FOOTER_KEY),
        Span::raw("  "),
        Span::styled("\u{23ce} select", theme::FOOTER_KEY),
        Span::raw("  "),
        Span::styled("? help", theme::FOOTER_KEY),
        Span::raw("  "),
        Span::styled("q quit", theme::FOOTER_KEY),
    ]);

    frame.render_widget(Paragraph::new(footer).style(theme::FOOTER_BG), area);
}
