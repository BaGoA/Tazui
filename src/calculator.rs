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

use std::collections::HashMap;

/// Process the string given by user
pub struct Calculator {
    variables: HashMap<String, f64>, // map to store variable name associated to its value
}

impl Calculator {
    /// Create a calculator
    pub fn new() -> Self {
        return Calculator {
            variables: HashMap::with_capacity(100),
        };
    }

    /// Process expression given in argument. This expression can use variable declared before.
    /// We return a pair (String, f64) which correspond to name of variable and its value.
    /// In case we have a raw expression, the name of variable is "last"
    /// If error occurs we return a string containing error message.
    pub fn process(&mut self, expression: &String) -> Result<(String, f64), String> {
        match expression.find('=') {
            Some(index) => {
                // Here we define a variable according to following format variable_name = variable_expression
                let name: String = String::from(expression.get(0..index).unwrap().trim());
                let variable_expression: String =
                    String::from(expression.get((index + 1)..).unwrap());

                match taz::evaluate_with_variables(&variable_expression, &self.variables) {
                    Ok(value) => {
                        self.variables.insert(name.clone(), value);
                        return Ok((name, value));
                    }
                    Err(message) => Err(message),
                }
            }
            None => {
                // Here we have raw expression
                match taz::evaluate_with_variables(&expression, &self.variables) {
                    Ok(value) => {
                        let name: String = String::from("last");
                        self.variables.insert(name.clone(), value);
                        return Ok((name, value));
                    }
                    Err(message) => Err(message),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn relative_error(value: f64, reference: f64) -> f64 {
        if reference == 0.0 {
            return value.abs();
        } else {
            return (value - reference).abs() / reference.abs();
        }
    }

    #[test]
    fn test_calculator_new() {
        let calc: Calculator = Calculator::new();
        assert!(calc.variables.capacity() > 0)
    }

    #[test]
    fn test_calculator_process_raw_expression() {
        let mut calc: Calculator = Calculator::new();
        let expression: String = String::from("cos(5.0)^2 + sin(5.0)^2");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("last"));
                assert!(relative_error(value, 1.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 1);
    }

    #[test]
    fn test_calculator_process_definition_of_variable() {
        let mut calc: Calculator = Calculator::new();
        let expression: String = String::from("t = cos(5.0)^2 + sin(5.0)^2");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("t"));
                assert!(relative_error(value, 1.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 1);
    }

    #[test]
    fn test_calculator_process_raw_expression_with_variable() {
        let mut calc: Calculator = Calculator::new();
        calc.variables.insert(String::from("t"), 5.0);

        let expression: String = String::from("cos(t)^2 + sin(t)^2");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("last"));
                assert!(relative_error(value, 1.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 2);
    }

    #[test]
    fn test_calculator_several_process() {
        let mut calc: Calculator = Calculator::new();
        let mut expression: String = String::from("t = 3.0 + 1.0");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("t"));
                assert!(relative_error(value, 4.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 1);

        expression = String::from("cos(t)^2 + sin(t)^2");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("last"));
                assert!(relative_error(value, 1.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 2);
    }
}
