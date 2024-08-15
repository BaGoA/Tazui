use std::collections::HashMap;

/// Process the string given by user
pub struct Calculator {
    variables: HashMap<String, f64>, // map to store custom variable defined by user
    formulas: HashMap<String, String>, // map to store formulas defined by user
}

impl Calculator {
    /// Create a calculator
    pub fn new() -> Self {
        return Calculator {
            variables: HashMap::with_capacity(100),
            formulas: HashMap::with_capacity(100),
        };
    }

    /// Update variables with new values of expressions
    fn update_variables_with_values_of_expressions(&mut self) -> Result<(), String> {
        for (name, expression) in &self.formulas {
            let value: f64 = taz::evaluate(&expression, &self.variables)?;
            self.variables.insert(name.clone(), value);
        }

        return Ok(());
    }

    /// Process expression given in argument. This expression can use variable declared before.
    /// We return a pair (String, f64) which correspond to name of variable and its value.
    /// In case we have a raw expression, the name of variable is "last"
    /// If error occurs we return a string containing error message.
    pub fn process(&mut self, expression: &String) -> Result<(String, f64), String> {
        self.update_variables_with_values_of_expressions()?;

        match expression.find('=') {
            Some(index) => {
                if index == 0 {
                    return Err(String::from(
                        "The expression is erroned, please enter a name for variable or expression",
                    ));
                }

                // Here we define a variable or formula according to following format:
                // - name = expression to define variable
                // - name := expression to define an formula
                let mut name: String = String::from(expression.get(0..index).unwrap().trim());
                let sub_expression: String = String::from(expression.get((index + 1)..).unwrap());

                if name.chars().last().unwrap() == ':' {
                    // In this case we define an formula
                    name.pop();

                    if name.is_empty() {
                        return Err(String::from(
                            "The expression is erroned, please enter name for expression",
                        ));
                    }

                    name = String::from(name.trim());

                    self.formulas.insert(name.clone(), sub_expression.clone());
                }

                let value: f64 = taz::evaluate(&sub_expression, &self.variables)?;

                self.variables.insert(name.clone(), value);

                return Ok((name, value));
            }
            None => {
                // Here we have raw expression
                let name: String = String::from("last");
                let value: f64 = taz::evaluate(&expression, &self.variables)?;

                self.variables.insert(name.clone(), value);

                return Ok((name, value));
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
        assert_eq!(calc.formulas.len(), 0);
    }

    #[test]
    fn test_calculator_process_definition_of_expression() {
        let mut calc: Calculator = Calculator::new();
        let expression: String = String::from("t := cos(5.0)^2 + sin(5.0)^2");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("t"));
                assert!(relative_error(value, 1.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 1);
        assert_eq!(calc.formulas.len(), 1);
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
    fn test_calculator_process_raw_expression_with_expression() {
        let mut calc: Calculator = Calculator::new();
        calc.variables.insert(String::from("t"), 5.0);

        calc.formulas
            .insert(String::from("s"), String::from("cos(t)^2 + sin(t)^2"));

        let expression: String = String::from("2.0 * s + 1.0");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("last"));
                assert!(relative_error(value, 3.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 3);
        assert_eq!(calc.formulas.len(), 1);
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
        assert_eq!(calc.formulas.len(), 0);

        expression = String::from("s := cos(t)^2 + sin(t)^2");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("s"));
                assert!(relative_error(value, 1.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 2);
        assert_eq!(calc.formulas.len(), 1);

        expression = String::from("2.0 * s + 1.0");

        match calc.process(&expression) {
            Ok((name, value)) => {
                assert_eq!(name, String::from("last"));
                assert!(relative_error(value, 3.0) < 1e-2);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(calc.variables.len(), 3);
        assert_eq!(calc.formulas.len(), 1);
    }
}
