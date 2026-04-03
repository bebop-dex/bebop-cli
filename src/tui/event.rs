use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::app::{ActiveTab, App};
use super::state::tokens_state::{InputMode, LoadState};

const POLL_TIMEOUT: Duration = Duration::from_millis(50);

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(POLL_TIMEOUT)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                handle_key(app, key);
            }
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    // Layer 1: Modal interceptors
    if app.tokens_state.chain_picker_open {
        handle_chain_picker_key(app, key);
        return;
    }

    if app.active_tab == ActiveTab::Tokens
        && app.tokens_state.input_mode == InputMode::Search
    {
        handle_search_key(app, key);
        return;
    }

    // Layer 2: Tab-specific keys
    if app.active_tab == ActiveTab::Tokens && handle_tokens_key(app, key) {
        return;
    }

    // Layer 3: Global keys
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Right => app.next_tab(),
        KeyCode::Left => app.prev_tab(),
        _ => {}
    }
}

fn handle_chain_picker_key(app: &mut App, key: KeyEvent) {
    let chain_count = app.tokens_state.chains.len();
    match key.code {
        KeyCode::Up => {
            if app.tokens_state.chain_picker_index > 0 {
                app.tokens_state.chain_picker_index -= 1;
            }
        }
        KeyCode::Down => {
            if chain_count > 0 && app.tokens_state.chain_picker_index < chain_count - 1 {
                app.tokens_state.chain_picker_index += 1;
            }
        }
        KeyCode::Enter => {
            if let Some(chain) = app
                .tokens_state
                .chains
                .get(app.tokens_state.chain_picker_index)
                .cloned()
            {
                app.tokens_state.selected_chain = chain;
                app.tokens_state.chain_picker_open = false;
                app.tokens_state.selected_index = 0;
                app.tokens_state.scroll_offset = 0;
                app.tokens_state.search_query.clear();
                app.request_tokens();
            }
        }
        KeyCode::Esc => {
            app.tokens_state.chain_picker_open = false;
        }
        _ => {}
    }
}

fn handle_search_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.tokens_state.exit_search();
        }
        KeyCode::Enter => {
            app.tokens_state.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.tokens_state.pop_search_char();
        }
        KeyCode::Up => {
            app.tokens_state.scroll_up();
        }
        KeyCode::Down => {
            app.tokens_state.scroll_down();
        }
        KeyCode::Char(c) => {
            app.tokens_state.push_search_char(c);
        }
        _ => {}
    }
}

/// Returns true if the key was consumed by the tokens tab.
fn handle_tokens_key(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('/') => {
            app.tokens_state.enter_search();
            true
        }
        KeyCode::Char('c') => {
            if app.tokens_state.chains_load_state != LoadState::Loaded {
                app.request_chains();
            }
            app.tokens_state.chain_picker_open = true;
            // Pre-select current chain in picker
            if let Some(pos) = app
                .tokens_state
                .chains
                .iter()
                .position(|c| c == &app.tokens_state.selected_chain)
            {
                app.tokens_state.chain_picker_index = pos;
            }
            true
        }
        KeyCode::Up => {
            app.tokens_state.scroll_up();
            true
        }
        KeyCode::Down => {
            app.tokens_state.scroll_down();
            true
        }
        _ => false,
    }
}
