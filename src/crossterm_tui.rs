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
use std::io::{stdout, Error, Stdout, Write};

use crossterm::{
    cursor::{MoveRight, MoveTo, MoveToColumn, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct CrosstermTui {
    stream: Stdout,
    pub start_of_line: String,
}

impl Default for CrosstermTui {
    fn default() -> Self {
        return CrosstermTui {
            stream: stdout(),
            start_of_line: String::from(">>> "),
        };
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
    fn init(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        execute!(self.stream, Clear(ClearType::All), MoveTo(0, 0))?;

        return Ok(());
    }

    fn get_start_of_line(&self) -> String {
        return self.start_of_line.clone();
    }

    fn get_expression(&mut self, history: &Vec<String>) -> Result<String, Error> {
        let mut expression: String = String::with_capacity(256);
        let mut history_iter = history.iter().rev(); // browse history from the end

        let mut pos: usize = 0;

        while let Ok(Event::Key(KeyEvent { code, .. })) = read() {
            match code {
                KeyCode::Char(c) => {
                    if pos == expression.len() {
                        expression.push(c);
                    } else {
                        expression.insert(pos, c);
                    }

                    pos += 1;

                    // Reset current line with updated expression and cursor position
                    execute!(self.stream, Clear(ClearType::CurrentLine), MoveToColumn(0))?;

                    write!(self.stream, "{}{}", self.start_of_line, expression)?;
                    self.stream.flush()?;

                    let new_cursor_column: usize = pos + self.start_of_line.len();

                    execute!(self.stream, MoveToColumn(new_cursor_column as u16))?;
                }
                KeyCode::Esc => {
                    expression = String::from("quit");
                    execute!(self.stream, MoveToNextLine(1))?;
                    break;
                }
                KeyCode::Enter => {
                    execute!(self.stream, MoveToNextLine(1))?;
                    break;
                }
                _ => continue,
            }
        }

        return Ok(expression);
    }

    fn display_text(&mut self, text: &String) -> Result<(), Error> {
        write!(self.stream, "{}", text)?;
        self.stream.flush()?;

        return Ok(());
    }

    fn display_text_with_new_line(&mut self, text: &String) -> Result<(), Error> {
        write!(self.stream, "{}", text)?;
        execute!(self.stream, MoveToNextLine(1))?;
        self.stream.flush()?;

        return Ok(());
    }
}
