use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs};

use super::app::{ActiveTab, App};
use super::state::theme;
use super::state::tokens_state::InputMode;

pub fn render(frame: &mut Frame, app: &mut App) {
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
    render_footer(frame, chunks[3], app);

    // Global chain picker — render on non-Config tabs (Config tab handles it internally)
    if app.config_state.global_chain_picker && app.active_tab != ActiveTab::Config {
        render_global_chain_picker(frame, frame.area(), app);
    }

    if app.help_overlay_open {
        super::widgets::help_overlay::render(frame, frame.area(), app.help_scroll_offset);
    }
}

fn render_global_chain_picker(frame: &mut Frame, area: Rect, app: &App) {
    use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState};

    let chains = &app.config_state.chains;
    let popup_width = 30u16.min(area.width.saturating_sub(4));
    let popup_height = (chains.len() as u16 + 2)
        .min(area.height.saturating_sub(4))
        .max(3);
    let popup_area = super::widgets::centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let items: Vec<ListItem> = chains.iter().map(|c| ListItem::new(c.as_str())).collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::CHAIN_PICKER_BORDER)
                .title(" Global Chain "),
        )
        .highlight_style(theme::CHAIN_PICKER_HIGHLIGHT)
        .highlight_symbol("\u{25b8} ");

    let mut list_state =
        ListState::default().with_selected(Some(app.config_state.chain_picker_index));
    frame.render_stateful_widget(list, popup_area, &mut list_state);
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

fn render_content(frame: &mut Frame, area: Rect, app: &mut App) {
    match app.active_tab {
        ActiveTab::Dashboard => {
            super::tabs::dashboard::render(frame, area, app);
        }
        ActiveTab::Tokens => {
            super::tabs::tokens::render(frame, area, app);
        }
        ActiveTab::Quote => {
            super::tabs::quote::render(frame, area, app);
        }
        ActiveTab::Config => {
            super::tabs::config::render(frame, area, app);
        }
    }
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    if app.quit_confirm {
        let footer = Line::from(vec![
            Span::raw(" "),
            Span::styled("press q again to quit", theme::FOOTER_KEY),
        ]);
        frame.render_widget(Paragraph::new(footer).style(theme::FOOTER_BG), area);
        return;
    }

    let hints: Vec<(&str, &str)> = if app.config_state.global_chain_picker {
        vec![
            ("\u{2191}\u{2193}", "navigate"),
            ("\u{23ce}", "select"),
            ("Esc", "cancel"),
        ]
    } else {
    match app.active_tab {
        // Tokens tab
        ActiveTab::Tokens if app.tokens_state.chain_picker_open => {
            vec![
                ("\u{2191}\u{2193}", "navigate"),
                ("\u{23ce}", "select"),
                ("Esc", "cancel"),
            ]
        }
        ActiveTab::Tokens if app.tokens_state.input_mode == InputMode::Search => {
            vec![
                ("type", "filter"),
                ("\u{2191}\u{2193}", "navigate"),
                ("\u{23ce}", "confirm"),
                ("Esc", "clear"),
            ]
        }
        ActiveTab::Tokens => {
            vec![
                ("\u{2190}\u{2192}", "tabs"),
                ("\u{2191}\u{2193}", "navigate"),
                ("/", "search"),
                ("c", "chain"),
                ("s", "sell"),
                ("b", "buy"),
                ("?", "help"),
                ("q", "quit"),
            ]
        }

        // Quote tab — modals
        ActiveTab::Quote if app.quote_state.delete_confirm => {
            vec![("y", "delete"), ("n/Esc", "cancel")]
        }
        ActiveTab::Quote if app.quote_state.chain_picker_open => {
            vec![
                ("\u{2191}\u{2193}", "navigate"),
                ("\u{23ce}", "select"),
                ("Esc", "cancel"),
            ]
        }
        ActiveTab::Quote if app.quote_state.token_search.open => {
            vec![
                ("type", "filter"),
                ("\u{2191}\u{2193}", "navigate"),
                ("\u{23ce}", "select"),
                ("Esc", "cancel"),
            ]
        }
        ActiveTab::Quote if app.quote_state.amount_editing => {
            vec![
                ("type", "enter amount"),
                ("\u{23ce}", "confirm"),
                ("Esc", "cancel"),
            ]
        }
        ActiveTab::Quote if app.quote_state.form_open => {
            vec![
                ("\u{2191}\u{2193}", "fields"),
                ("\u{23ce}", "select"),
                ("c", "chain"),
                ("Esc", "close"),
            ]
        }
        // Quote tab — table view
        ActiveTab::Quote => {
            vec![
                ("\u{2190}\u{2192}", "tabs"),
                ("\u{2191}\u{2193}", "navigate"),
                ("n", "new quote"),
                ("d", "delete"),
                ("?", "help"),
                ("q", "quit"),
            ]
        }

        // Config tab
        ActiveTab::Config if app.config_state.chain_picker_open => {
            vec![
                ("\u{2191}\u{2193}", "navigate"),
                ("\u{23ce}", "select"),
                ("Esc", "cancel"),
            ]
        }
        ActiveTab::Config if app.config_state.editing => {
            vec![
                ("type", "edit"),
                ("\u{23ce}", "confirm"),
                ("Esc", "cancel"),
            ]
        }
        ActiveTab::Config => {
            vec![
                ("\u{2190}\u{2192}", "tabs"),
                ("\u{2191}\u{2193}", "fields"),
                ("\u{23ce}", "edit"),
                ("?", "help"),
                ("q", "quit"),
            ]
        }

        // Dashboard
        ActiveTab::Dashboard => {
            vec![
                ("\u{2190}\u{2192}", "tabs"),
                ("Shift+C", "chain"),
                ("?", "help"),
                ("q", "quit"),
            ]
        }
    }
    };

    let spans: Vec<Span> = hints
        .iter()
        .enumerate()
        .flat_map(|(i, (key, action))| {
            let mut s = vec![Span::styled(
                format!("{key} {action}"),
                theme::FOOTER_KEY,
            )];
            if i < hints.len() - 1 {
                s.push(Span::raw("  "));
            }
            s
        })
        .collect();

    let mut line_spans = vec![Span::raw(" ")];
    line_spans.extend(spans);
    let footer = Line::from(line_spans);

    frame.render_widget(Paragraph::new(footer).style(theme::FOOTER_BG), area);
}
