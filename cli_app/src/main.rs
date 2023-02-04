use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use file_handler;
use std::path::Path;

use dirs;
fn main() -> Result<(), io::Error> {
    let from = dirs::home_dir().unwrap().as_path().join("/Rust/slot_em_all");
    let to = dirs::home_dir().unwrap().as_path().join("./slot_you");
    file_handler::copy_recursively(Path::new(from.as_path()), Path::new(to.as_path()), Some(false))?;

    Ok(())
}

// fn main() -> Result<(), io::Error> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     terminal.draw(|f| {
//         let size = f.size();
//         let block = Block::default()
//             .title("Ayy Lmao")
//             .borders(Borders::LEFT | Borders::RIGHT);
//         f.render_widget(block, size);
//     })?;

//     thread::sleep(Duration::from_millis(5000));

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     Ok(())
// }