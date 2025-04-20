mod application;
mod crossterm_tui;
mod tui;

use application::Application;
use crossterm_tui::CrosstermTui;

fn main() {
    let mut app = Application::new(CrosstermTui::default(), taz::evaluate);

    if let Err(error) = app.init() {
        println!("{}\n", error);
    }

    match app.run() {
        Ok(()) => (),
        Err(error) => println!("{}\n", error),
    }
}
