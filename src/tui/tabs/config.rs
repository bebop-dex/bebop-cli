use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::config_state::ConfigField;
use crate::tui::state::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let rows = Layout::vertical([
        Constraint::Length(1), // Title
        Constraint::Length(1), // spacer
        Constraint::Length(1), // API Key
        Constraint::Length(1), // Wallet Address
        Constraint::Length(1), // Default Chain
        Constraint::Length(1), // Output Format
        Constraint::Length(1), // spacer
        Constraint::Length(1), // error / status
        Constraint::Min(0),
    ])
    .split(area);

    // Title
    let title = Line::from(Span::styled(
        " Config Settings",
        theme::CONFIG_LABEL,
    ));
    frame.render_widget(Paragraph::new(title), rows[0]);

    // API Key
    render_field(
        frame,
        rows[2],
        "API Key:        ",
        &format_api_key(app),
        ConfigField::ApiKey,
        app,
    );

    // Wallet Address
    render_field(
        frame,
        rows[3],
        "Wallet Address: ",
        &format_wallet(app),
        ConfigField::WalletAddress,
        app,
    );

    // Default Chain
    let chain_val = match &app.config.chain {
        Some(c) => format!("[{c} \u{25be}]"),
        None => "[not set \u{25be}]".to_string(),
    };
    render_field(
        frame,
        rows[4],
        "Default Chain:  ",
        &chain_val,
        ConfigField::DefaultChain,
        app,
    );

    // Output Format
    let current_fmt = app.config.format.as_deref().unwrap_or("json");
    let (json_ind, text_ind) = if current_fmt == "text" {
        ("\u{25cb}", "\u{25cf}")
    } else {
        ("\u{25cf}", "\u{25cb}")
    };
    let fmt_val = format!("[{json_ind} json  {text_ind} text]");
    render_field(
        frame,
        rows[5],
        "Output Format:  ",
        &fmt_val,
        ConfigField::OutputFormat,
        app,
    );

    // Error / status line
    if let Some(ref err) = app.config_state.save_error {
        let err_line = Line::from(vec![
            Span::raw("  "),
            Span::styled(err.as_str(), theme::CONFIG_ERROR),
        ]);
        frame.render_widget(Paragraph::new(err_line), rows[7]);
    }

    // Chain picker overlay
    if app.config_state.chain_picker_open || app.config_state.global_chain_picker {
        render_chain_picker(frame, area, app);
    }
}

fn render_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    field: ConfigField,
    app: &App,
) {
    let is_active = app.config_state.active_field == field;
    let value_style = if is_active {
        theme::CONFIG_ACTIVE
    } else {
        theme::CONFIG_VALUE
    };

    let line = Line::from(vec![
        Span::styled(format!(" {label}"), theme::CONFIG_LABEL),
        Span::styled(value, value_style),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

fn format_api_key(app: &App) -> String {
    if app.config_state.editing && app.config_state.active_field == ConfigField::ApiKey {
        format!("[{}\u{258e}]", app.config_state.draft)
    } else {
        match &app.config.api_key {
            Some(key) => format!("[{}]", "*".repeat(key.len().min(20))),
            None => "[not set]".to_string(),
        }
    }
}

fn format_wallet(app: &App) -> String {
    if app.config_state.editing && app.config_state.active_field == ConfigField::WalletAddress {
        format!("[{}\u{258e}]", app.config_state.draft)
    } else {
        match &app.config.wallet_address {
            Some(addr) if addr.len() > 12 => {
                format!("[{}...{}]", &addr[..6], &addr[addr.len() - 4..])
            }
            Some(addr) => format!("[{addr}]"),
            None => "[not set]".to_string(),
        }
    }
}

fn render_chain_picker(frame: &mut Frame, area: Rect, app: &App) {
    let chains = &app.config_state.chains;
    let popup_width = 30u16.min(area.width.saturating_sub(4));
    let popup_height = (chains.len() as u16 + 2)
        .min(area.height.saturating_sub(4))
        .max(3);
    let popup_area = super::super::widgets::centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let items: Vec<ratatui::widgets::ListItem> = chains
        .iter()
        .map(|c| ratatui::widgets::ListItem::new(c.as_str()))
        .collect();

    let title = if app.config_state.global_chain_picker {
        " Global Chain "
    } else {
        " Select Chain "
    };

    let list = ratatui::widgets::List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::CHAIN_PICKER_BORDER)
                .title(title),
        )
        .highlight_style(theme::CHAIN_PICKER_HIGHLIGHT)
        .highlight_symbol("\u{25b8} ");

    let mut list_state =
        ratatui::widgets::ListState::default().with_selected(Some(app.config_state.chain_picker_index));
    frame.render_stateful_widget(list, popup_area, &mut list_state);
}
