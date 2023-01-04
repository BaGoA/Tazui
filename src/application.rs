/*
   Marvin is simple terminal calculator
   Copyright (C) 2022  Bastian Gonzalez Acevedo

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::calculator::Calculator;
use super::tui::Tui;

use std::io::Error;

/// Taz calculator application
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

        self.tui.display_text_with_new_line(&String::from(
            "Marvin Copyright (C) 2022 Bastian Gonzalez Acevedo",
        ))?;

        self.tui.display_text_with_new_line(&String::from(
            "This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.",
        ))?;

        self.tui.display_text_with_new_line(&String::from("This is free software, and you are welcome to redistribute it under certain conditions; type `show c' for details."))?;

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
