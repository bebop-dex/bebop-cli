pub mod app;
pub mod event;
pub mod state;
pub mod tabs;
pub mod ui;
pub mod widgets;

use app::App;

pub fn run() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = run_loop(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run_loop(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    while app.running {
        terminal.draw(|frame| ui::render(frame, app))?;
        event::handle_events(app)?;
    }
    Ok(())
}
