mod app;

use std::io::stdout;
use color_eyre::Result;
use app::App;
use ratatui::{
    crossterm::{
        terminal::{
            EnterAlternateScreen, LeaveAlternateScreen,
        },
        execute,
    },
    layout::Rect,
    TerminalOptions,
    Viewport
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let viewport = Viewport::Fixed(Rect::new(0, 0, 81, 18));
    let terminal = ratatui::init_with_options(TerminalOptions { viewport });
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let app_result = App::default().run(terminal);
    execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    ratatui::restore();
    app_result
}
