use crate::tokens::Token;

pub enum AppMessage {
    TokensLoaded { chain: String, tokens: Vec<Token> },
    TokensError { chain: String, error: String },
    ChainsLoaded { chains: Vec<String> },
    ChainsError { error: String },
}
