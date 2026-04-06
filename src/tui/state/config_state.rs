use super::LoadState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigField {
    ApiKey,
    WalletAddress,
    DefaultChain,
    OutputFormat,
}

impl ConfigField {
    pub fn next(self) -> Self {
        match self {
            Self::ApiKey => Self::WalletAddress,
            Self::WalletAddress => Self::DefaultChain,
            Self::DefaultChain => Self::OutputFormat,
            Self::OutputFormat => Self::ApiKey,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::ApiKey => Self::OutputFormat,
            Self::WalletAddress => Self::ApiKey,
            Self::DefaultChain => Self::WalletAddress,
            Self::OutputFormat => Self::DefaultChain,
        }
    }
}

pub struct ConfigState {
    pub active_field: ConfigField,
    pub editing: bool,
    pub draft: String,

    // Chain picker (reused pattern from tokens/quote)
    pub chain_picker_open: bool,
    pub chains: Vec<String>,
    pub chains_load_state: LoadState,
    pub chain_picker_index: usize,

    // Global chain picker (Shift+C from any tab)
    pub global_chain_picker: bool,

    // Transient error display
    pub save_error: Option<String>,

    pub initialized: bool,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            active_field: ConfigField::ApiKey,
            editing: false,
            draft: String::new(),

            chain_picker_open: false,
            chains: Vec::new(),
            chains_load_state: LoadState::Idle,
            chain_picker_index: 0,

            global_chain_picker: false,

            save_error: None,

            initialized: false,
        }
    }

    pub fn next_field(&mut self) {
        self.active_field = self.active_field.next();
    }

    pub fn prev_field(&mut self) {
        self.active_field = self.active_field.prev();
    }

    pub fn start_editing(&mut self, current_value: &str) {
        self.editing = true;
        self.draft = current_value.to_string();
        self.save_error = None;
    }

    pub fn cancel_editing(&mut self) {
        self.editing = false;
        self.draft.clear();
    }
}
