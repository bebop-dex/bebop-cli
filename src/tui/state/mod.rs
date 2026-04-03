pub mod quote_state;
pub mod theme;
pub mod tokens_state;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadState {
    Idle,
    Loading,
    Loaded,
    Error(String),
}
