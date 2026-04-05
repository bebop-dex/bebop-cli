use serde::{Deserialize, Serialize};

use crate::tokens::Token;

use super::LoadState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteSide {
    Sell,
    Buy,
}

impl QuoteSide {
    #[allow(dead_code)] // used in epoch 4+ for display
    pub fn label(self) -> &'static str {
        match self {
            Self::Sell => "Sell",
            Self::Buy => "Buy",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormField {
    Chain,
    SellToken,
    BuyToken,
    Amount,
    Side,
    Submit,
}

impl FormField {
    pub fn next(self) -> Self {
        match self {
            Self::Chain => Self::SellToken,
            Self::SellToken => Self::BuyToken,
            Self::BuyToken => Self::Amount,
            Self::Amount => Self::Side,
            Self::Side => Self::Submit,
            Self::Submit => Self::Chain,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Chain => Self::Submit,
            Self::SellToken => Self::Chain,
            Self::BuyToken => Self::SellToken,
            Self::Amount => Self::BuyToken,
            Self::Side => Self::Amount,
            Self::Submit => Self::Side,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenSearchTarget {
    SellToken,
    BuyToken,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteEntryStatus {
    Active,
    Expired,
    Error(String),
}

pub struct TokenSearchState {
    pub open: bool,
    pub target: TokenSearchTarget,
    pub query: String,
    pub tokens: Vec<Token>,
    pub filtered_indices: Vec<usize>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub list_height: usize,
}

impl TokenSearchState {
    pub fn new() -> Self {
        Self {
            open: false,
            target: TokenSearchTarget::SellToken,
            query: String::new(),
            tokens: Vec::new(),
            filtered_indices: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            list_height: 0,
        }
    }

    pub fn open_for(&mut self, target: TokenSearchTarget) {
        self.open = true;
        self.target = target;
        self.query.clear();
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.refilter();
    }

    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
    }

    pub fn refilter(&mut self) {
        if self.query.is_empty() {
            self.filtered_indices = (0..self.tokens.len()).collect();
        } else {
            let mut scored: Vec<(usize, u64)> = self
                .tokens
                .iter()
                .enumerate()
                .filter_map(|(i, t)| {
                    crate::tokens::search_score(t, &self.query).map(|s| (i, s))
                })
                .collect();
            scored.sort_by_key(|&(_, s)| s);
            self.filtered_indices = scored.into_iter().map(|(i, _)| i).collect();
        }
        if self.selected_index >= self.filtered_indices.len() {
            self.selected_index = self.filtered_indices.len().saturating_sub(1);
        }
        self.scroll_offset = self.scroll_offset.min(self.selected_index);
    }

    pub fn scroll_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }

    pub fn scroll_down(&mut self) {
        let count = self.filtered_indices.len();
        if count > 0 && self.selected_index < count - 1 {
            self.selected_index += 1;
            if self.list_height > 0 && self.selected_index >= self.scroll_offset + self.list_height {
                self.scroll_offset = self.selected_index - self.list_height + 1;
            }
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.refilter();
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.refilter();
    }

    pub fn selected_token(&self) -> Option<&Token> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|&i| self.tokens.get(i))
    }
}

pub struct QuoteEntry {
    // Request params (persisted)
    pub request_amount: String,
    pub request_side: QuoteSide,

    // Response data (transient)
    pub sell_token: Token,
    pub buy_token: Token,
    pub sell_amount: String,
    pub buy_amount: String,
    pub rate: Option<f64>,
    pub price_impact: Option<f64>,
    pub expiry: u64,
    pub chain: String,
    pub status: QuoteEntryStatus,
}

pub struct QuoteState {
    // Form fields
    pub selected_chain: String,
    pub sell_token: Option<Token>,
    pub buy_token: Option<Token>,
    pub amount: String,
    pub side: QuoteSide,

    // Form navigation
    pub active_field: FormField,
    pub amount_editing: bool,

    // Chain picker
    pub chain_picker_open: bool,
    pub chains: Vec<String>,
    pub chains_load_state: LoadState,
    pub chain_picker_index: usize,

    // Token search modal
    pub token_search: TokenSearchState,

    // Token data for the form's chain
    pub form_tokens_load_state: LoadState,

    // Quote submission
    pub quote_load_state: LoadState,
    pub quote_error: Option<String>,
    pub spinner_tick: usize,

    // Results table
    pub entries: Vec<QuoteEntry>,
    pub table_selected: usize,
    pub table_scroll_offset: usize,
    pub table_height: usize,

    // Delete confirmation
    pub delete_confirm: bool,

    // Form modal
    pub form_open: bool,

    pub initialized: bool,
}

impl QuoteState {
    pub fn new(default_chain: Option<&str>) -> Self {
        Self {
            selected_chain: default_chain.unwrap_or("ethereum").to_string(),
            sell_token: None,
            buy_token: None,
            amount: String::new(),
            side: QuoteSide::Sell,

            active_field: FormField::Chain,
            amount_editing: false,

            chain_picker_open: false,
            chains: Vec::new(),
            chains_load_state: LoadState::Idle,
            chain_picker_index: 0,

            token_search: TokenSearchState::new(),

            form_tokens_load_state: LoadState::Idle,

            quote_load_state: LoadState::Idle,
            quote_error: None,
            spinner_tick: 0,

            entries: Vec::new(),
            table_selected: 0,
            table_scroll_offset: 0,
            table_height: 0,

            delete_confirm: false,

            form_open: false,

            initialized: false,
        }
    }

    pub fn next_field(&mut self) {
        self.active_field = self.active_field.next();
    }

    pub fn prev_field(&mut self) {
        self.active_field = self.active_field.prev();
    }

    pub fn toggle_side(&mut self) {
        self.side = match self.side {
            QuoteSide::Sell => QuoteSide::Buy,
            QuoteSide::Buy => QuoteSide::Sell,
        };
    }

    pub fn clear_form(&mut self) {
        self.sell_token = None;
        self.buy_token = None;
        self.amount.clear();
        self.side = QuoteSide::Sell;
        self.active_field = FormField::Chain;
        self.quote_error = None;
    }

    pub fn can_submit(&self) -> bool {
        self.sell_token.is_some()
            && self.buy_token.is_some()
            && !self.amount.is_empty()
            && self.amount.parse::<f64>().map(|v| v > 0.0).unwrap_or(false)
    }

    pub fn table_scroll_up(&mut self) {
        if self.table_selected > 0 {
            self.table_selected -= 1;
            if self.table_selected < self.table_scroll_offset {
                self.table_scroll_offset = self.table_selected;
            }
        }
    }

    pub fn table_scroll_down(&mut self) {
        if !self.entries.is_empty() && self.table_selected < self.entries.len() - 1 {
            self.table_selected += 1;
            if self.table_height > 0
                && self.table_selected >= self.table_scroll_offset + self.table_height
            {
                self.table_scroll_offset = self.table_selected - self.table_height + 1;
            }
        }
    }
}
