pub mod app;
pub mod event;
pub mod message;
pub mod quote_store;
pub mod state;
pub mod tabs;
pub mod ui;
pub mod widgets;

use app::App;
use message::AppMessage;
use state::LoadState;

pub async fn run() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let mut app = App::new(tx.clone());

    // Load persisted quotes and fire initial re-quotes
    app.load_persisted_quotes();

    // Spawn the 15-second auto-refresh timer
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        interval.tick().await; // consume the immediate first tick
        loop {
            interval.tick().await;
            if tx.send(AppMessage::RefreshTick).is_err() {
                break;
            }
        }
    });

    let result = run_loop(&mut terminal, &mut app, rx);
    ratatui::restore();
    result
}

fn run_loop(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<AppMessage>,
) -> std::io::Result<()> {
    while app.running {
        terminal.draw(|frame| ui::render(frame, app))?;
        event::handle_events(app)?;

        while let Ok(msg) = rx.try_recv() {
            app.handle_message(msg);
        }

        if app.tokens_state.load_state == LoadState::Loading {
            app.tokens_state.spinner_tick = app.tokens_state.spinner_tick.wrapping_add(1);
        }

        if app.quote_state.quote_load_state == LoadState::Loading
            || app.quote_state.form_tokens_load_state == LoadState::Loading
        {
            app.quote_state.spinner_tick = app.quote_state.spinner_tick.wrapping_add(1);
        }
    }
    Ok(())
}
