use crate::tokens::Token;

pub enum AppMessage {
    TokensLoaded { chain: String, tokens: Vec<Token> },
    TokensError { chain: String, error: String },
    ChainsLoaded { chains: Vec<String> },
    ChainsError { error: String },

    // Quote tab
    QuoteTokensLoaded { chain: String, tokens: Vec<Token> },
    QuoteTokensError { chain: String, error: String },
    QuoteLoaded {
        sell_token: Token,
        buy_token: Token,
        sell_amount: String,
        buy_amount: String,
        rate: Option<f64>,
        price_impact: Option<f64>,
        expiry: u64,
        chain: String,
    },
    QuoteError { error: String },

    // Auto-refresh (epoch 4)
    RefreshTick,
    RefreshQuoteLoaded {
        index: usize,
        sell_amount: String,
        buy_amount: String,
        rate: Option<f64>,
        price_impact: Option<f64>,
        expiry: u64,
    },
    RefreshQuoteError {
        index: usize,
        error: String,
    },
}
