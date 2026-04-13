use anyhow::Result;
use crate::application::runner;
use crate::domain::App;
use crate::infrastructure::terminal;

mod application;
mod domain;
mod infrastructure;
mod presentation;

fn main() -> Result<()> {
    // 1. Terminal Panic Hook (Absolute Fault Tolerance)
    // Ensures terminal restoration even if the app panics.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = terminal::restore();
        original_hook(panic_info);
    }));

    // 2. TUI Initialization
    let mut tui_terminal = terminal::init()?;
    let app = App::new();
    
    // 3. Application Execution
    let result = runner::run(&mut tui_terminal, app);

    // 4. Normal Cleanup
    terminal::restore()?;
    
    result
}
