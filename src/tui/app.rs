use tokio::sync::mpsc::UnboundedSender;

use crate::config::Config;

use super::message::AppMessage;
use super::state::tokens_state::{LoadState, TokensState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Dashboard,
    Tokens,
    Quote,
    Config,
}

impl ActiveTab {
    pub const ALL: [ActiveTab; 4] = [
        Self::Dashboard,
        Self::Tokens,
        Self::Quote,
        Self::Config,
    ];

    pub fn next(self) -> Self {
        match self {
            Self::Dashboard => Self::Tokens,
            Self::Tokens => Self::Quote,
            Self::Quote => Self::Config,
            Self::Config => Self::Dashboard,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Dashboard => Self::Config,
            Self::Tokens => Self::Dashboard,
            Self::Quote => Self::Tokens,
            Self::Config => Self::Quote,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Tokens => "Tokens",
            Self::Quote => "Quote",
            Self::Config => "Config",
        }
    }
}

pub struct App {
    pub active_tab: ActiveTab,
    pub running: bool,
    pub config: Config,
    pub tokens_state: TokensState,
    pub tx: UnboundedSender<AppMessage>,
}

impl App {
    pub fn new(tx: UnboundedSender<AppMessage>) -> Self {
        let config = Config::load();
        let tokens_state = TokensState::new(config.chain.as_deref());
        Self {
            active_tab: ActiveTab::Dashboard,
            running: true,
            config,
            tokens_state,
            tx,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = self.active_tab.next();
        self.check_tokens_init();
    }

    pub fn prev_tab(&mut self) {
        self.active_tab = self.active_tab.prev();
        self.check_tokens_init();
    }

    pub fn check_tokens_init(&mut self) {
        if self.active_tab == ActiveTab::Tokens && !self.tokens_state.initialized {
            self.tokens_state.initialized = true;
            self.request_tokens();
            self.request_chains();
        }
    }

    pub fn request_tokens(&mut self) {
        let chain = self.tokens_state.selected_chain.clone();
        self.tokens_state.load_state = LoadState::Loading;
        self.tokens_state.spinner_tick = 0;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            match crate::tokens::try_fetch_tokens(&chain).await {
                Ok(tokens) => {
                    let _ = tx.send(AppMessage::TokensLoaded { chain, tokens });
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::TokensError {
                        chain,
                        error: e,
                    });
                }
            }
        });
    }

    pub fn request_chains(&mut self) {
        self.tokens_state.chains_load_state = LoadState::Loading;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            match crate::chains::try_fetch().await {
                Ok(map) => {
                    let chains: Vec<String> = map.keys().cloned().collect();
                    let _ = tx.send(AppMessage::ChainsLoaded { chains });
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::ChainsError { error: e });
                }
            }
        });
    }

    pub fn handle_message(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::TokensLoaded { chain, tokens } => {
                if chain == self.tokens_state.selected_chain {
                    self.tokens_state.tokens = tokens;
                    self.tokens_state.load_state = LoadState::Loaded;
                    self.tokens_state.selected_index = 0;
                    self.tokens_state.scroll_offset = 0;
                    self.tokens_state.refilter();
                }
            }
            AppMessage::TokensError { chain, error } => {
                if chain == self.tokens_state.selected_chain {
                    self.tokens_state.load_state = LoadState::Error(error);
                }
            }
            AppMessage::ChainsLoaded { chains } => {
                self.tokens_state.chains = chains;
                self.tokens_state.chains_load_state = LoadState::Loaded;
            }
            AppMessage::ChainsError { error } => {
                self.tokens_state.chains_load_state = LoadState::Error(error);
            }
        }
    }
}
