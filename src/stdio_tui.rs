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
use std::io::{Stdin, Stdout, Write};

pub struct StdIoTui {
    reader: Stdin,
    writer: Stdout,
}

impl Tui for StdIoTui {
    fn new() -> Self {
        return StdIoTui {
            reader: std::io::stdin(),
            writer: std::io::stdout(),
        };
    }

    fn get_expression(&mut self, history: &Vec<String>) -> Result<String, String> {
        let mut expression: String = String::with_capacity(256);

        match self.reader.read_line(&mut expression) {
            Ok(_) => Ok(expression),
            Err(error) => Err(error.to_string()),
        }
    }

    fn display_string(&mut self, string: &String) {
        match self.writer.write(string.as_bytes()) {
            Ok(_) => (),
            Err(_) => (),
        }

        match self.writer.flush() {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}
