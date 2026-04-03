use crate::tokens::Token;

pub use super::LoadState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
}

pub struct TokensState {
    pub tokens: Vec<Token>,
    pub load_state: LoadState,
    pub selected_chain: String,

    pub chains: Vec<String>,
    pub chains_load_state: LoadState,
    pub chain_picker_open: bool,
    pub chain_picker_index: usize,

    pub selected_index: usize,
    pub scroll_offset: usize,
    pub table_height: usize,

    pub input_mode: InputMode,
    pub search_query: String,
    pub filtered_indices: Vec<usize>,

    pub spinner_tick: usize,
    pub initialized: bool,
}

impl TokensState {
    pub fn new(default_chain: Option<&str>) -> Self {
        Self {
            tokens: Vec::new(),
            load_state: LoadState::Idle,
            selected_chain: default_chain.unwrap_or("ethereum").to_string(),
            chains: Vec::new(),
            chains_load_state: LoadState::Idle,
            chain_picker_open: false,
            chain_picker_index: 0,
            selected_index: 0,
            scroll_offset: 0,
            table_height: 0,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            filtered_indices: Vec::new(),
            spinner_tick: 0,
            initialized: false,
        }
    }

    pub fn refilter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices = (0..self.tokens.len()).collect();
        } else {
            let q = self.search_query.to_lowercase();
            self.filtered_indices = self
                .tokens
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    t.symbol.to_lowercase().contains(&q)
                        || t.name.to_lowercase().contains(&q)
                        || t.address.to_lowercase().contains(&q)
                })
                .map(|(i, _)| i)
                .collect();
        }
        if self.selected_index >= self.filtered_indices.len() {
            self.selected_index = self.filtered_indices.len().saturating_sub(1);
        }
        self.scroll_offset = self.scroll_offset.min(self.selected_index);
    }

    pub fn visible_count(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn selected_token(&self) -> Option<&Token> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|&i| self.tokens.get(i))
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
        if self.visible_count() > 0 && self.selected_index < self.visible_count() - 1 {
            self.selected_index += 1;
            if self.table_height > 0 && self.selected_index >= self.scroll_offset + self.table_height
            {
                self.scroll_offset = self.selected_index - self.table_height + 1;
            }
        }
    }

    pub fn enter_search(&mut self) {
        self.input_mode = InputMode::Search;
        self.search_query.clear();
        self.refilter();
    }

    pub fn exit_search(&mut self) {
        self.input_mode = InputMode::Normal;
        self.search_query.clear();
        self.refilter();
    }

    pub fn push_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.refilter();
    }

    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.refilter();
    }
}
