use tokio::sync::mpsc::UnboundedSender;

use crate::config::Config;

use super::message::AppMessage;
use super::state::LoadState;
use super::state::quote_state::{QuoteEntry, QuoteEntryStatus, QuoteState};
use super::state::tokens_state::TokensState;

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
    pub quote_state: QuoteState,
    pub tx: UnboundedSender<AppMessage>,
}

impl App {
    pub fn new(tx: UnboundedSender<AppMessage>) -> Self {
        let config = Config::load();
        let tokens_state = TokensState::new(config.chain.as_deref());
        let quote_state = QuoteState::new(config.chain.as_deref());
        Self {
            active_tab: ActiveTab::Dashboard,
            running: true,
            config,
            tokens_state,
            quote_state,
            tx,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = self.active_tab.next();
        self.check_lazy_init();
    }

    pub fn prev_tab(&mut self) {
        self.active_tab = self.active_tab.prev();
        self.check_lazy_init();
    }

    fn check_lazy_init(&mut self) {
        self.check_tokens_init();
        self.check_quote_init();
    }

    pub fn check_tokens_init(&mut self) {
        if self.active_tab == ActiveTab::Tokens && !self.tokens_state.initialized {
            self.tokens_state.initialized = true;
            self.request_tokens();
            self.request_chains();
        }
    }

    pub fn check_quote_init(&mut self) {
        if self.active_tab == ActiveTab::Quote && !self.quote_state.initialized {
            self.quote_state.initialized = true;
            self.request_quote_tokens();
            self.request_quote_chains();
        }
    }

    // --- Tokens tab requests ---

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

    // --- Quote tab requests ---

    pub fn request_quote_tokens(&mut self) {
        let chain = self.quote_state.selected_chain.clone();
        self.quote_state.form_tokens_load_state = LoadState::Loading;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            match crate::tokens::try_fetch_tokens(&chain).await {
                Ok(tokens) => {
                    let _ = tx.send(AppMessage::QuoteTokensLoaded { chain, tokens });
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::QuoteTokensError { chain, error: e });
                }
            }
        });
    }

    pub fn request_quote_chains(&mut self) {
        self.quote_state.chains_load_state = LoadState::Loading;
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

    pub fn request_quote(&mut self) {
        let sell_token = match &self.quote_state.sell_token {
            Some(t) => t.clone(),
            None => return,
        };
        let buy_token = match &self.quote_state.buy_token {
            Some(t) => t.clone(),
            None => return,
        };
        let amount = self.quote_state.amount.clone();
        if amount.is_empty() {
            return;
        }
        let side = self.quote_state.side;
        let chain = self.quote_state.selected_chain.clone();
        let wallet = self.config.wallet_address.clone()
            .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string());
        let api_key = self.config.api_key.clone();

        self.quote_state.quote_load_state = LoadState::Loading;
        self.quote_state.quote_error = None;
        self.quote_state.spinner_tick = 0;

        let tx = self.tx.clone();
        tokio::spawn(async move {
            use crate::quote::firm::{fetch_quote, from_base_units};
            use super::state::quote_state::QuoteSide;

            let (amount_buy, amount_sell) = match side {
                QuoteSide::Sell => (None, Some(amount.as_str())),
                QuoteSide::Buy => (Some(amount.as_str()), None),
            };

            match fetch_quote(
                &buy_token,
                &sell_token,
                amount_buy,
                amount_sell,
                &chain,
                &wallet,
                api_key.as_deref(),
            )
            .await
            {
                Ok(response) => {
                    let sell_resp = response.sell_tokens.values().next();
                    let buy_resp = response.buy_tokens.values().next();

                    if let (Some(sr), Some(br)) = (sell_resp, buy_resp) {
                        let sell_amount = from_base_units(&sr.amount, sr.decimals);
                        let buy_amount = from_base_units(&br.amount, br.decimals);
                        let sell_f = sell_amount.parse::<f64>().unwrap_or(0.0);
                        let buy_f = buy_amount.parse::<f64>().unwrap_or(0.0);
                        let rate = if sell_f > 0.0 { Some(buy_f / sell_f) } else { None };

                        let _ = tx.send(AppMessage::QuoteLoaded {
                            sell_token,
                            buy_token,
                            sell_amount,
                            buy_amount,
                            rate,
                            price_impact: response.price_impact,
                            expiry: response.expiry,
                            chain,
                        });
                    } else {
                        let _ = tx.send(AppMessage::QuoteError {
                            error: "Missing token data in response".to_string(),
                        });
                    }
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::QuoteError { error: e });
                }
            }
        });
    }

    // --- Message handling ---

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
                self.tokens_state.chains = chains.clone();
                self.tokens_state.chains_load_state = LoadState::Loaded;
                self.quote_state.chains = chains;
                self.quote_state.chains_load_state = LoadState::Loaded;
            }
            AppMessage::ChainsError { error } => {
                self.tokens_state.chains_load_state = LoadState::Error(error.clone());
                self.quote_state.chains_load_state = LoadState::Error(error);
            }

            // Quote tab
            AppMessage::QuoteTokensLoaded { chain, tokens } => {
                if chain == self.quote_state.selected_chain {
                    self.quote_state.token_search.tokens = tokens;
                    self.quote_state.form_tokens_load_state = LoadState::Loaded;
                    self.quote_state.token_search.refilter();
                }
            }
            AppMessage::QuoteTokensError { chain, error } => {
                if chain == self.quote_state.selected_chain {
                    self.quote_state.form_tokens_load_state = LoadState::Error(error);
                }
            }
            AppMessage::QuoteLoaded {
                sell_token,
                buy_token,
                sell_amount,
                buy_amount,
                rate,
                price_impact,
                expiry,
                chain,
            } => {
                self.quote_state.entries.push(QuoteEntry {
                    sell_token,
                    buy_token,
                    sell_amount,
                    buy_amount,
                    rate,
                    price_impact,
                    expiry,
                    chain,
                    status: QuoteEntryStatus::Active,
                });
                self.quote_state.quote_load_state = LoadState::Loaded;
                self.quote_state.table_selected = self.quote_state.entries.len().saturating_sub(1);
                self.quote_state.form_open = false;
                self.quote_state.clear_form();
            }
            AppMessage::QuoteError { error } => {
                self.quote_state.quote_load_state = LoadState::Error(error.clone());
                self.quote_state.quote_error = Some(error);
            }
        }
    }
}
