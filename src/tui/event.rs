use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::app::{ActiveTab, App};
use super::state::LoadState;
use super::state::quote_state::{FormField, TokenSearchTarget};
use super::state::tokens_state::InputMode;

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
    // Layer 1: Modal interceptors — tokens tab
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

    // Layer 1: Modal interceptors — quote tab
    if app.active_tab == ActiveTab::Quote {
        if app.quote_state.delete_confirm {
            handle_quote_delete_confirm(app, key);
            return;
        }
        if app.quote_state.form_open {
            // Sub-modals within the form
            if app.quote_state.chain_picker_open {
                handle_quote_chain_picker_key(app, key);
                return;
            }
            if app.quote_state.token_search.open {
                handle_token_search_key(app, key);
                return;
            }
            if app.quote_state.amount_editing {
                handle_quote_amount_input(app, key);
                return;
            }
            // Form modal keys
            handle_quote_form_key(app, key);
            return;
        }
    }

    // Layer 2: Tab-specific keys
    if app.active_tab == ActiveTab::Tokens && handle_tokens_key(app, key) {
        return;
    }
    if app.active_tab == ActiveTab::Quote && handle_quote_key(app, key) {
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

// --- Tokens tab handlers ---

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

// --- Quote tab handlers ---

fn handle_quote_delete_confirm(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            let idx = app.quote_state.table_selected;
            if idx < app.quote_state.entries.len() {
                app.quote_state.entries.remove(idx);
                if app.quote_state.table_selected >= app.quote_state.entries.len()
                    && app.quote_state.table_selected > 0
                {
                    app.quote_state.table_selected -= 1;
                }
            }
            app.quote_state.delete_confirm = false;
            app.save_quotes();
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.quote_state.delete_confirm = false;
        }
        _ => {}
    }
}

fn handle_quote_chain_picker_key(app: &mut App, key: KeyEvent) {
    let chain_count = app.quote_state.chains.len();
    match key.code {
        KeyCode::Up => {
            if app.quote_state.chain_picker_index > 0 {
                app.quote_state.chain_picker_index -= 1;
            }
        }
        KeyCode::Down => {
            if chain_count > 0 && app.quote_state.chain_picker_index < chain_count - 1 {
                app.quote_state.chain_picker_index += 1;
            }
        }
        KeyCode::Enter => {
            if let Some(chain) = app
                .quote_state
                .chains
                .get(app.quote_state.chain_picker_index)
                .cloned()
            {
                app.quote_state.selected_chain = chain;
                app.quote_state.chain_picker_open = false;
                // Clear token selections — new chain has different tokens
                app.quote_state.sell_token = None;
                app.quote_state.buy_token = None;
                app.request_quote_tokens();
            }
        }
        KeyCode::Esc => {
            app.quote_state.chain_picker_open = false;
        }
        _ => {}
    }
}

fn handle_token_search_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.quote_state.token_search.close();
        }
        KeyCode::Enter => {
            if let Some(token) = app.quote_state.token_search.selected_token().cloned() {
                match app.quote_state.token_search.target {
                    TokenSearchTarget::SellToken => {
                        app.quote_state.sell_token = Some(token);
                    }
                    TokenSearchTarget::BuyToken => {
                        app.quote_state.buy_token = Some(token);
                    }
                }
                app.quote_state.token_search.close();
            }
        }
        KeyCode::Backspace => {
            app.quote_state.token_search.pop_char();
        }
        KeyCode::Up => {
            app.quote_state.token_search.scroll_up();
        }
        KeyCode::Down => {
            app.quote_state.token_search.scroll_down();
        }
        KeyCode::Char(c) => {
            app.quote_state.token_search.push_char(c);
        }
        _ => {}
    }
}

fn handle_quote_amount_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.quote_state.amount_editing = false;
        }
        KeyCode::Backspace => {
            app.quote_state.amount.pop();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            app.quote_state.amount.push(c);
        }
        KeyCode::Char('.') => {
            if !app.quote_state.amount.contains('.') {
                app.quote_state.amount.push('.');
            }
        }
        _ => {}
    }
}

/// Returns true if the key was consumed by the quote tab (table view).
fn handle_quote_key(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('n') => {
            app.quote_state.form_open = true;
            app.quote_state.active_field = FormField::Chain;
            app.quote_state.quote_error = None;
            true
        }
        KeyCode::Up => {
            app.quote_state.table_scroll_up();
            true
        }
        KeyCode::Down => {
            app.quote_state.table_scroll_down();
            true
        }
        KeyCode::Char('d') => {
            if !app.quote_state.entries.is_empty() {
                app.quote_state.delete_confirm = true;
            }
            true
        }
        _ => false,
    }
}

/// Handles keys when the form modal is open.
fn handle_quote_form_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.quote_state.form_open = false;
        }
        KeyCode::Up => {
            app.quote_state.prev_field();
        }
        KeyCode::Down => {
            app.quote_state.next_field();
        }
        KeyCode::Enter => {
            handle_form_field_activate(app);
        }
        KeyCode::Char('c') => {
            open_quote_chain_picker(app);
        }
        _ => {}
    }
}

fn handle_form_field_activate(app: &mut App) {
    match app.quote_state.active_field {
        FormField::Chain => {
            open_quote_chain_picker(app);
        }
        FormField::SellToken => {
            if app.quote_state.form_tokens_load_state == LoadState::Idle {
                app.request_quote_tokens();
            }
            app.quote_state
                .token_search
                .open_for(TokenSearchTarget::SellToken);
        }
        FormField::BuyToken => {
            if app.quote_state.form_tokens_load_state == LoadState::Idle {
                app.request_quote_tokens();
            }
            app.quote_state
                .token_search
                .open_for(TokenSearchTarget::BuyToken);
        }
        FormField::Amount => {
            app.quote_state.amount_editing = true;
        }
        FormField::Side => {
            app.quote_state.toggle_side();
        }
        FormField::Submit => {
            if app.quote_state.can_submit() && app.quote_state.quote_load_state != LoadState::Loading
            {
                app.quote_state.quote_error = None;
                app.request_quote();
            }
        }
    }
}

fn open_quote_chain_picker(app: &mut App) {
    if app.quote_state.chains_load_state != LoadState::Loaded {
        app.request_quote_chains();
    }
    app.quote_state.chain_picker_open = true;
    if let Some(pos) = app
        .quote_state
        .chains
        .iter()
        .position(|c| c == &app.quote_state.selected_chain)
    {
        app.quote_state.chain_picker_index = pos;
    }
}
