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

use super::tui::Tui;
use super::calculator::Calculator;

/// Taz calculator application
pub struct Application<TuiApp>
where TuiApp: Tui,
{
    tui: TuiApp,
    calculator : Calculator,
    history: Vec<String>,
}

impl<TuiApp: Tui> Application<TuiApp> {
    /// Create a application
    pub fn new() -> Self {
        return Application {
            tui: TuiApp::new(),
            calculator: Calculator::new(),
            history: Vec::with_capacity(5),
        };
    }

    /// Run the application
    pub fn run(&mut self) {
        let header: String = 
            String::from("Marvin Copyright (C) 2022 Bastian Gonzalez Acevedo\nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.\nThis is free software, and you are welcome to redistribute it under certain conditions; type `show c' for details.\n\n");

        self.tui.display_string(&header);

        let start_expression: String = String::from(">>> ");
        
        loop {
            self.tui.display_string(&start_expression);

            let collected_expression : Result<String, String> = self.tui.get_expression(&self.history);
            
            if collected_expression.is_err() {
                self.tui.display_string(&collected_expression.err().unwrap());
                break;
            }
            
            let expression : String = String::from(collected_expression.unwrap().trim_matches('\n'));
            
            if expression == String::from("quit") {
                break;
            }

            if expression.len() == 0 {
                continue;
            }
    
            let str_result: String = match self.calculator.process(&expression) {
                Ok((name, value)) => format!("{} = {}\n", name, value),
                Err(message) => format!("{}\n", message)
            };

            self.tui.display_string(&str_result);
            self.history.push(expression);
        }
    }
}
