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
    variables: HashMap<String, f64>,
}

impl Calculator {
    /// Create a calculator
    pub fn new() -> Self {
        return Calculator {
            variables: HashMap::with_capacity(100),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_new() {
        let calc: Calculator = Calculator::new();
        assert!(calc.variables.capacity() > 0)
    }
}
