use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::tui::state::quote_state::{TokenSearchState, TokenSearchTarget};
use crate::tui::state::theme;
use crate::tui::state::LoadState;
use crate::tui::widgets::spinner;

pub fn render(frame: &mut Frame, area: Rect, state: &TokenSearchState, tokens_load_state: &LoadState, spinner_tick: usize) {
    let popup_width = 50u16.min(area.width.saturating_sub(4));
    let popup_height = 20u16.min(area.height.saturating_sub(4)).max(5);
    let popup_area = super::centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let title = match state.target {
        TokenSearchTarget::SellToken => " Select Sell Token ",
        TokenSearchTarget::BuyToken => " Select Buy Token ",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::TOKEN_SEARCH_BORDER)
        .title(title);

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(inner);

    // Search input
    let search_line = Line::from(vec![
        Span::styled(" / ", theme::TOKEN_SEARCH_PROMPT),
        Span::styled(&state.query, theme::TOKEN_SEARCH_INPUT),
        Span::styled("\u{258e}", theme::TOKEN_SEARCH_INPUT),
    ]);
    frame.render_widget(Paragraph::new(search_line), chunks[0]);

    // Token list or loading/error state
    match tokens_load_state {
        LoadState::Loading => {
            let spin = spinner::spinner(spinner_tick);
            frame.render_widget(Paragraph::new(Line::from(vec![Span::raw(" "), spin])), chunks[1]);
        }
        LoadState::Error(err) => {
            let msg = Paragraph::new(format!(" Error: {err}")).style(theme::QUOTE_IMPACT_HIGH);
            frame.render_widget(msg, chunks[1]);
        }
        _ => {
            let items: Vec<ListItem> = state
                .filtered_indices
                .iter()
                .skip(state.scroll_offset)
                .filter_map(|&i| state.tokens.get(i))
                .map(|t| ListItem::new(format!(" {:10} {}", t.symbol, t.name)))
                .collect();

            if items.is_empty() {
                let msg = Paragraph::new(" No tokens found").style(theme::QUOTE_FORM_PLACEHOLDER);
                frame.render_widget(msg, chunks[1]);
            } else {
                let display_index = state.selected_index.saturating_sub(state.scroll_offset);
                let list = List::new(items)
                    .highlight_style(theme::TOKEN_SEARCH_HIGHLIGHT)
                    .highlight_symbol("\u{25b8} ");

                let mut list_state = ListState::default().with_selected(Some(display_index));
                frame.render_stateful_widget(list, chunks[1], &mut list_state);
            }
        }
    }
}
