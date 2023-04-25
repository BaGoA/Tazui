use super::calculator::Calculator;
use super::tui::Tui;

use std::io::Error;

/// Tazui calculator application
pub struct Application<TuiApp>
where
    TuiApp: Tui + Default,
{
    tui: TuiApp,            // terminal user interface to interact with user
    calculator: Calculator, // calculator to process expressions given by user
    history: Vec<String>,   // store previous expressions enter by user
}

impl<TuiApp: Tui + Default> Application<TuiApp> {
    /// Create a application
    pub fn new() -> Self {
        return Application {
            tui: TuiApp::default(),
            calculator: Calculator::new(),
            history: Vec::with_capacity(5),
        };
    }

    /// Initialization of application
    /// We initialize TUI and write application copyright
    pub fn init(&mut self) -> Result<(), Error> {
        self.tui.init()?;

        self.tui
            .display_text_with_new_line(&String::from("Tazui Calculator"))?;

        return Ok(());
    }

    /// Run the application
    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            // Get expression given by user
            self.tui.display_text(&self.tui.get_start_of_line())?;

            let expression: String = self.tui.get_expression(&self.history)?;

            if expression == String::from("quit") {
                break;
            }

            if expression.len() == 0 {
                continue;
            }

            // Process and display the result of expression
            match self.calculator.process(&expression) {
                Ok((name, value)) => {
                    self.history.push(expression);
                    self.tui
                        .display_text_with_new_line(&format!("{} = {}", name, value))?;
                }
                Err(message) => {
                    self.tui.display_text_with_new_line(&message)?;
                }
            };
        }

        return Ok(());
    }
}
