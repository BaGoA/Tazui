mod application;
mod tui;

use application::Application;

fn main() {
    // Evaluator using taz
    let evaluator_fn = |expression: &str| {
        taz::evaluate(expression).map_err(|taz_err: taz::error::Error| taz_err.message())
    };

    let mut app = Application::new(tui::Tui::default(), evaluator_fn);

    if let Err(error) = app.init() {
        println!("{}\n", error);
    }

    match app.run() {
        Ok(()) => (),
        Err(error) => println!("{}\n", error),
    }
}
