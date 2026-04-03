use serde::{Deserialize, Serialize};

use crate::tokens::Token;

use super::state::quote_state::QuoteSide;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistedQuote {
    pub sell_token: Token,
    pub buy_token: Token,
    pub amount: String,
    pub side: QuoteSide,
    pub chain: String,
}

pub fn load() -> Vec<PersistedQuote> {
    let path = crate::cache::cache_dir().join("quotes.json");
    let Ok(data) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save(entries: &[PersistedQuote]) {
    let dir = crate::cache::cache_dir();
    let _ = std::fs::create_dir_all(&dir);
    let Ok(json) = serde_json::to_string_pretty(entries) else {
        return;
    };
    let _ = std::fs::write(dir.join("quotes.json"), json);
}
