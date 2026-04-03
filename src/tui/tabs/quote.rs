use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::LoadState;
use crate::tui::state::quote_state::{FormField, QuoteEntryStatus, QuoteSide};
use crate::tui::state::theme;
use crate::tui::widgets::{spinner, token_search};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    render_table(frame, area, app);

    // Overlay modals — order matters (later = on top)
    if app.quote_state.form_open {
        render_form_modal(frame, area, app);

        if app.quote_state.chain_picker_open {
            render_quote_chain_picker(frame, area, app);
        }
        if app.quote_state.token_search.open {
            token_search::render(
                frame,
                area,
                &app.quote_state.token_search,
                &app.quote_state.form_tokens_load_state,
                app.quote_state.spinner_tick,
            );
        }
    }

    if app.quote_state.delete_confirm {
        render_delete_confirm(frame, area);
    }
}

fn render_table(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::QUOTE_FORM_BORDER)
        .title(" Quotes ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.quote_state.entries.is_empty() {
        let msg = Paragraph::new("  No quotes yet. Press n to create a new quote.")
            .style(theme::QUOTE_FORM_PLACEHOLDER);
        frame.render_widget(msg, inner);
        return;
    }

    let header_height = 1u16;
    let table_content_height = inner.height.saturating_sub(header_height);
    app.quote_state.table_height = table_content_height as usize;

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let rows: Vec<Row> = app
        .quote_state
        .entries
        .iter()
        .enumerate()
        .skip(app.quote_state.table_scroll_offset)
        .map(|(i, entry)| {
            let sell_col = format!("{} {}", entry.sell_amount, entry.sell_token.symbol);
            let buy_col = format!("{} {}", entry.buy_amount, entry.buy_token.symbol);

            let rate_col = match entry.rate {
                Some(r) => format!("{:.6}", r),
                None => "-".to_string(),
            };

            let (impact_col, impact_style) = match entry.price_impact {
                Some(p) => {
                    let pct = p * 100.0;
                    let style = if pct.abs() < 0.5 {
                        theme::QUOTE_IMPACT_LOW
                    } else if pct.abs() < 2.0 {
                        theme::QUOTE_IMPACT_MED
                    } else {
                        theme::QUOTE_IMPACT_HIGH
                    };
                    (format!("{:.2}%", pct), style)
                }
                None => ("-".to_string(), theme::QUOTE_TABLE_ROW),
            };

            let ttl = entry.expiry as i64 - now_secs as i64;
            let (ttl_col, ttl_style) = if ttl <= 0 {
                ("expired".to_string(), theme::QUOTE_TTL_EXPIRED)
            } else {
                (format!("{}s", ttl), theme::QUOTE_TTL_OK)
            };

            let status_col = match &entry.status {
                QuoteEntryStatus::Active if ttl <= 0 => "\u{23f3}",
                QuoteEntryStatus::Active => "\u{2713}",
                QuoteEntryStatus::Expired => "\u{23f3}",
                QuoteEntryStatus::Error(_) => "\u{26a0}",
            };

            let row_style = if i == app.quote_state.table_selected {
                theme::QUOTE_TABLE_SELECTED
            } else {
                theme::QUOTE_TABLE_ROW
            };

            Row::new(vec![
                Cell::from(sell_col),
                Cell::from(buy_col),
                Cell::from(rate_col),
                Cell::from(impact_col).style(impact_style),
                Cell::from(ttl_col).style(ttl_style),
                Cell::from(entry.chain.clone()),
                Cell::from(status_col),
            ])
            .style(row_style)
        })
        .collect();

    let header = Row::new(vec!["Sell", "Buy", "Rate", "Impact", "TTL", "Chain", ""])
        .style(theme::QUOTE_TABLE_HEADER);

    let widths = [
        Constraint::Percentage(18),
        Constraint::Percentage(18),
        Constraint::Percentage(20),
        Constraint::Percentage(12),
        Constraint::Percentage(10),
        Constraint::Percentage(14),
        Constraint::Percentage(8),
    ];

    let table = Table::new(rows, widths).header(header);

    let display_selected = app
        .quote_state
        .table_selected
        .saturating_sub(app.quote_state.table_scroll_offset);
    let mut table_state = TableState::default().with_selected(Some(display_selected));

    frame.render_stateful_widget(table, inner, &mut table_state);
}

fn render_form_modal(frame: &mut Frame, area: Rect, app: &App) {
    let popup_width = 44u16.min(area.width.saturating_sub(4));
    let popup_height = 10u16.min(area.height.saturating_sub(4)).max(8);
    let popup_area = super::super::widgets::centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::TOKEN_SEARCH_BORDER)
        .title(" New Quote ");

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let rows = Layout::vertical([
        Constraint::Length(1), // Chain
        Constraint::Length(1), // Sell Token
        Constraint::Length(1), // Buy Token
        Constraint::Length(1), // Amount
        Constraint::Length(1), // Side
        Constraint::Length(1), // spacer
        Constraint::Length(1), // Submit / status
        Constraint::Min(0),
    ])
    .split(inner);

    // Chain
    let chain_style = field_style(FormField::Chain, &app.quote_state.active_field);
    let chain_line = Line::from(vec![
        Span::styled(" Chain:      ", theme::QUOTE_FORM_LABEL),
        Span::styled(
            format!("[{} \u{25be}]", app.quote_state.selected_chain),
            chain_style,
        ),
    ]);
    frame.render_widget(Paragraph::new(chain_line), rows[0]);

    // Sell Token
    let sell_style = field_style(FormField::SellToken, &app.quote_state.active_field);
    let sell_text = match &app.quote_state.sell_token {
        Some(t) => format!("[{}]", t.symbol),
        None => "[select...]".to_string(),
    };
    let sell_value_style = if app.quote_state.sell_token.is_some() || app.quote_state.active_field == FormField::SellToken {
        sell_style
    } else {
        theme::QUOTE_FORM_PLACEHOLDER
    };
    let sell_line = Line::from(vec![
        Span::styled(" Sell Token: ", theme::QUOTE_FORM_LABEL),
        Span::styled(sell_text, sell_value_style),
    ]);
    frame.render_widget(Paragraph::new(sell_line), rows[1]);

    // Buy Token
    let buy_style = field_style(FormField::BuyToken, &app.quote_state.active_field);
    let buy_text = match &app.quote_state.buy_token {
        Some(t) => format!("[{}]", t.symbol),
        None => "[select...]".to_string(),
    };
    let buy_value_style = if app.quote_state.buy_token.is_some() || app.quote_state.active_field == FormField::BuyToken {
        buy_style
    } else {
        theme::QUOTE_FORM_PLACEHOLDER
    };
    let buy_line = Line::from(vec![
        Span::styled(" Buy Token:  ", theme::QUOTE_FORM_LABEL),
        Span::styled(buy_text, buy_value_style),
    ]);
    frame.render_widget(Paragraph::new(buy_line), rows[2]);

    // Amount
    let amount_style = field_style(FormField::Amount, &app.quote_state.active_field);
    let (amount_text, amount_value_style) = if app.quote_state.amount.is_empty() {
        if app.quote_state.amount_editing {
            ("[_]".to_string(), amount_style)
        } else if app.quote_state.active_field == FormField::Amount {
            ("[enter amount...]".to_string(), amount_style)
        } else {
            ("[enter amount...]".to_string(), theme::QUOTE_FORM_PLACEHOLDER)
        }
    } else if app.quote_state.amount_editing {
        (format!("[{}\u{258e}]", app.quote_state.amount), amount_style)
    } else {
        (format!("[{}]", app.quote_state.amount), amount_style)
    };
    let amount_line = Line::from(vec![
        Span::styled(" Amount:     ", theme::QUOTE_FORM_LABEL),
        Span::styled(amount_text, amount_value_style),
    ]);
    frame.render_widget(Paragraph::new(amount_line), rows[3]);

    // Side
    let side_style = field_style(FormField::Side, &app.quote_state.active_field);
    let (sell_indicator, buy_indicator) = match app.quote_state.side {
        QuoteSide::Sell => ("\u{25cf} Sell", "\u{25cb} Buy"),
        QuoteSide::Buy => ("\u{25cb} Sell", "\u{25cf} Buy"),
    };
    let side_line = Line::from(vec![
        Span::styled(" Side:       ", theme::QUOTE_FORM_LABEL),
        Span::styled(format!("[{sell_indicator}  {buy_indicator}]"), side_style),
    ]);
    frame.render_widget(Paragraph::new(side_line), rows[4]);

    // Submit / status
    let submit_line = if app.quote_state.quote_load_state == LoadState::Loading {
        Line::from(vec![
            Span::raw(" "),
            spinner::spinner(app.quote_state.spinner_tick),
        ])
    } else if let Some(ref err) = app.quote_state.quote_error {
        Line::from(vec![
            Span::raw(" "),
            Span::styled(
                truncate_str(err, popup_width.saturating_sub(4) as usize),
                theme::QUOTE_IMPACT_HIGH,
            ),
        ])
    } else {
        let submit_style = if app.quote_state.active_field == FormField::Submit {
            if app.quote_state.can_submit() {
                theme::QUOTE_SUBMIT_READY
            } else {
                theme::QUOTE_SUBMIT_DISABLED
            }
        } else if app.quote_state.can_submit() {
            theme::QUOTE_FORM_VALUE
        } else {
            theme::QUOTE_SUBMIT_DISABLED
        };
        Line::from(vec![
            Span::raw(" "),
            Span::styled("[ Get Quote ]", submit_style),
        ])
    };
    frame.render_widget(Paragraph::new(submit_line), rows[6]);
}

fn render_quote_chain_picker(frame: &mut Frame, area: Rect, app: &App) {
    let chains = &app.quote_state.chains;
    let popup_width = 30u16.min(area.width.saturating_sub(4));
    let popup_height = (chains.len() as u16 + 2)
        .min(area.height.saturating_sub(4))
        .max(3);
    let popup_area = super::super::widgets::centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let items: Vec<ratatui::widgets::ListItem> = chains
        .iter()
        .map(|c| ratatui::widgets::ListItem::new(c.as_str()))
        .collect();

    let list = ratatui::widgets::List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::CHAIN_PICKER_BORDER)
                .title(" Select Chain "),
        )
        .highlight_style(theme::CHAIN_PICKER_HIGHLIGHT)
        .highlight_symbol("\u{25b8} ");

    let mut list_state =
        ratatui::widgets::ListState::default().with_selected(Some(app.quote_state.chain_picker_index));
    frame.render_stateful_widget(list, popup_area, &mut list_state);
}

fn render_delete_confirm(frame: &mut Frame, area: Rect) {
    let popup_area = super::super::widgets::centered_rect(34, 3, area);
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::QUOTE_DELETE_PROMPT);

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let line = Line::from(vec![
        Span::styled(" Delete this quote? ", theme::QUOTE_DELETE_PROMPT),
        Span::styled("(y/n)", theme::QUOTE_FORM_VALUE),
    ]);
    frame.render_widget(Paragraph::new(line), inner);
}

fn field_style(field: FormField, active: &FormField) -> ratatui::style::Style {
    if field == *active {
        theme::QUOTE_FORM_ACTIVE
    } else {
        theme::QUOTE_FORM_VALUE
    }
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
