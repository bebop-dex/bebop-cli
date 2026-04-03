use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState};
use ratatui::Frame;

use crate::tui::state::theme;
use crate::tui::state::tokens_state::TokensState;

pub fn render(frame: &mut Frame, area: Rect, state: &TokensState) {
    let popup_width = 30u16.min(area.width.saturating_sub(4));
    let popup_height = (state.chains.len() as u16 + 2).min(area.height.saturating_sub(4)).max(3);
    let popup_area = centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let items: Vec<ListItem> = state
        .chains
        .iter()
        .map(|c| ListItem::new(c.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::CHAIN_PICKER_BORDER)
                .title(" Select Chain "),
        )
        .highlight_style(theme::CHAIN_PICKER_HIGHLIGHT)
        .highlight_symbol("▸ ");

    let mut list_state = ListState::default().with_selected(Some(state.chain_picker_index));
    frame.render_stateful_widget(list, popup_area, &mut list_state);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    let horizontal = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0]);
    horizontal[0]
}
