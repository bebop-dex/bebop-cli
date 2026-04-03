pub mod app;
pub mod event;
pub mod message;
pub mod state;
pub mod tabs;
pub mod ui;
pub mod widgets;

use app::App;
use message::AppMessage;
use state::tokens_state::LoadState;

pub async fn run() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let mut app = App::new(tx);
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
    }
    Ok(())
}
