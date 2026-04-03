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

    render_header(frame, chunks[0]);
    render_tab_bar(frame, chunks[1], app.active_tab);
    render_content(frame, chunks[2], app.active_tab);
    render_footer(frame, chunks[3]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let version = env!("CARGO_PKG_VERSION");

    let header = Line::from(vec![
        Span::styled(format!(" bebop-cli v{version}"), theme::HEADER_TITLE),
        Span::raw("    "),
        Span::styled("[ethereum \u{25be}]", theme::HEADER_CHAIN),
        Span::raw("    "),
        Span::styled("wallet \u{2713}", theme::HEADER_STATUS),
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

fn render_content(frame: &mut Frame, area: Rect, active: ActiveTab) {
    let text = match active {
        ActiveTab::Dashboard => "Dashboard \u{2014} overview coming soon",
        ActiveTab::Tokens => "Tokens \u{2014} token browser coming soon",
        ActiveTab::Quote => "Quote \u{2014} RFQ interface coming soon",
        ActiveTab::Config => "Config \u{2014} settings editor coming soon",
    };

    let content = Paragraph::new(text).style(theme::CONTENT_TEXT);
    frame.render_widget(content, area);
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
