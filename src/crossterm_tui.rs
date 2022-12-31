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
use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct CrosstermTui {
    stream: Stdout,
}

impl Default for CrosstermTui {
    fn default() -> Self {
        return CrosstermTui { stream: stdout() };
    }
}

impl Drop for CrosstermTui {
    fn drop(&mut self) {
        match disable_raw_mode() {
            Ok(()) => (),
            Err(error) => println!("Cannot disable raw mode: {}\n", error.to_string()),
        }
    }
}

impl Tui for CrosstermTui {
    fn init(&mut self) -> Result<(), String> {
        if let Err(error) = enable_raw_mode() {
            return Err(error.to_string());
        }

        match execute!(self.stream, Clear(ClearType::All), MoveTo(0, 0)) {
            Ok(()) => return Ok(()),
            Err(error) => return Err(error.to_string()),
        }
    }

    fn get_expression(&mut self, history: &Vec<String>) -> Result<String, String> {
        //let mut expression: String = String::with_capacity(256);
        return Ok(String::from("quit"));
    }

    fn display_text(&mut self, text: &String) -> Result<(), String> {
        match write!(self.stream, "{}", text) {
            Ok(()) => return Ok(()),
            Err(error) => return Err(error.to_string()),
        }
    }

    fn display_text_with_new_line(&mut self, text: &String) -> Result<(), String> {
        if let Err(error) = write!(self.stream, "{}", text) {
            return Err(error.to_string());
        }

        match execute!(self.stream, MoveToNextLine(1)) {
            Ok(()) => return Ok(()),
            Err(error) => return Err(error.to_string()),
        }
    }
}
