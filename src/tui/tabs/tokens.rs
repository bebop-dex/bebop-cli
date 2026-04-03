use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::theme;
use crate::tui::state::tokens_state::{InputMode, LoadState};
use crate::tui::widgets::{chain_picker, spinner};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(1), // chain bar
        Constraint::Length(1), // search bar or column headers
        Constraint::Min(1),    // token table
    ])
    .split(area);

    render_chain_bar(frame, chunks[0], app);
    render_search_or_headers(frame, chunks[1], app);
    render_table(frame, chunks[2], app);

    if app.tokens_state.chain_picker_open {
        chain_picker::render(frame, area, &app.tokens_state);
    }
}

fn render_chain_bar(frame: &mut Frame, area: Rect, app: &App) {
    let count = app.tokens_state.visible_count();
    let total = app.tokens_state.tokens.len();
    let count_text = if app.tokens_state.search_query.is_empty() {
        format!("{total} tokens")
    } else {
        format!("{count}/{total} tokens")
    };

    let line = Line::from(vec![
        Span::raw(" Chain: "),
        Span::styled(
            format!("[{} ▾]", app.tokens_state.selected_chain),
            theme::TOKENS_CHAIN_VALUE,
        ),
        Span::raw("    "),
        Span::styled(count_text, theme::TOKENS_COUNT),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}

fn render_search_or_headers(frame: &mut Frame, area: Rect, app: &App) {
    match app.tokens_state.input_mode {
        InputMode::Search => {
            let line = Line::from(vec![
                Span::styled(" /", theme::TOKENS_SEARCH_PROMPT),
                Span::styled(&app.tokens_state.search_query, theme::TOKENS_SEARCH_INPUT),
                Span::styled("▎", theme::TOKENS_SEARCH_INPUT),
            ]);
            frame.render_widget(Paragraph::new(line), area);
        }
        InputMode::Normal => {
            let header = Line::from(vec![
                Span::styled(" Symbol", theme::TOKENS_HEADER),
                Span::raw("         "),
                Span::styled("Name", theme::TOKENS_HEADER),
                Span::raw("                            "),
                Span::styled("Address", theme::TOKENS_HEADER),
                Span::raw("                                 "),
                Span::styled("Dec", theme::TOKENS_HEADER),
            ]);
            frame.render_widget(Paragraph::new(header), area);
        }
    }
}

fn render_table(frame: &mut Frame, area: Rect, app: &mut App) {
    let table_height = area.height.saturating_sub(2) as usize; // minus borders
    app.tokens_state.table_height = table_height;

    match &app.tokens_state.load_state {
        LoadState::Idle => {
            let msg = Paragraph::new("Press Right arrow to load tokens")
                .style(theme::CONTENT_TEXT)
                .block(Block::default().borders(Borders::ALL).border_style(theme::TOKENS_BORDER));
            frame.render_widget(msg, area);
        }
        LoadState::Loading => {
            let spin = spinner::spinner(app.tokens_state.spinner_tick);
            let msg = Paragraph::new(Line::from(spin))
                .block(Block::default().borders(Borders::ALL).border_style(theme::TOKENS_BORDER));
            frame.render_widget(msg, area);
        }
        LoadState::Error(err) => {
            let msg = Paragraph::new(format!(" Error: {err}"))
                .style(ratatui::style::Style::new().fg(ratatui::style::Color::Red))
                .block(Block::default().borders(Borders::ALL).border_style(theme::TOKENS_BORDER));
            frame.render_widget(msg, area);
        }
        LoadState::Loaded => {
            render_token_table(frame, area, app);
        }
    }
}

fn truncate_address(addr: &str) -> String {
    if addr.len() > 14 {
        format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
    } else {
        addr.to_string()
    }
}

fn render_token_table(frame: &mut Frame, area: Rect, app: &mut App) {
    let state = &app.tokens_state;

    let rows: Vec<Row> = state
        .filtered_indices
        .iter()
        .skip(state.scroll_offset)
        .filter_map(|&i| state.tokens.get(i))
        .map(|t| {
            Row::new(vec![
                Cell::from(t.symbol.as_str()),
                Cell::from(t.name.as_str()),
                Cell::from(truncate_address(&t.address)).style(theme::TOKENS_ADDRESS),
                Cell::from(t.decimals.to_string()),
            ])
            .style(theme::TOKENS_ROW)
        })
        .collect();

    let widths = [
        Constraint::Percentage(15),
        Constraint::Percentage(35),
        Constraint::Percentage(40),
        Constraint::Percentage(10),
    ];

    let table = Table::new(rows, widths)
        .header(
            Row::new(vec![
                Cell::from("Symbol"),
                Cell::from("Name"),
                Cell::from("Address"),
                Cell::from("Dec"),
            ])
            .style(theme::TOKENS_HEADER),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::TOKENS_BORDER),
        )
        .row_highlight_style(theme::TOKENS_SELECTED);

    let display_index = if state.selected_index >= state.scroll_offset {
        state.selected_index - state.scroll_offset
    } else {
        0
    };

    let mut table_state = TableState::default().with_selected(Some(display_index));
    frame.render_stateful_widget(table, area, &mut table_state);
}
