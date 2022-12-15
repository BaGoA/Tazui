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

use super::ui::Tui;
use super::calculator::Calculator;

/// Taz calculator application
pub struct Application<UIApp>
where
    UIApp: Tui,
{
    ui: UIApp,
    calculator : Calculator,
    history: Vec<String>,
}

impl<UIApp: Tui> Application<UIApp> {
    /// Create a application
    pub fn new() -> Self {
        return Application {
            ui: UIApp::new(),
            calculator: Calculator::new(),
            history: Vec::with_capacity(5),
        };
    }

    /// Run the application
    pub fn run(&mut self) {
        let header: String = 
            String::from("Marvin Copyright (C) 2022 Bastian Gonzalez Acevedo\nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.\nThis is free software, and you are welcome to redistribute it under certain conditions; type `show c' for details.\n\n");

        self.ui.display_string(&header);

        let start_expression: String = String::from(">>> ");
        
        loop {
            self.ui.display_string(&start_expression);

            let expression: String = self.ui.get_expression(&self.history);

            if expression == String::from("quit") {
                break;
            }

            if expression.len() == 0 {
                continue;
            }
            
            match self.calculator.process(&expression) {
                Ok((name, value)) => {
                    let str_result: String = format!("{} = {}", name, value);
                    self.ui.display_string(&str_result);
                }
                Err(message) => self.ui.display_string(&message)
            }

            self.ui.display_string(&String::from('\n'));

            self.history.push(expression);
        }
    }
}
