use crate::config::Config;

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
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::Dashboard,
            running: true,
            config: Config::load(),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = self.active_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.active_tab = self.active_tab.prev();
    }
}
