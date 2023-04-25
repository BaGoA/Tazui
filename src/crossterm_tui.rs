use super::tui::Tui;
use std::io::{stdout, Error, Stdout, Write};

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

// TUI implementation using crossterm
pub struct CrosstermTui {
    stream: Stdout,
    pub start_of_line: String,
}

impl CrosstermTui {
    // Reset current line with updated expression and cursor position given in argument
    fn update_current_line(&mut self, expression: &String, cursor_pos: usize) -> Result<(), Error> {
        execute!(self.stream, Clear(ClearType::CurrentLine), MoveToColumn(0))?;

        write!(self.stream, "{}{}", self.start_of_line, expression)?;
        self.stream.flush()?;

        let new_cursor_pos: usize = cursor_pos + self.start_of_line.len();
        execute!(self.stream, MoveToColumn(new_cursor_pos as u16))?;

        return Ok(());
    }
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
    // Initialization of TUI
    // We clear terminal and put cursor to begin
    fn init(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        execute!(self.stream, Clear(ClearType::All), MoveTo(0, 0))?;

        return Ok(());
    }

    // Get string which represent start of line.
    fn get_start_of_line(&self) -> String {
        return self.start_of_line.clone();
    }

    // Get expression enter by user
    // We handle keyboard input to create the expression to process
    fn get_expression(&mut self, history: &Vec<String>) -> Result<String, Error> {
        let mut expression: String = String::with_capacity(256);
        let mut history_iter = history.iter().rev(); // browse history from the end

        let mut cursor_pos: usize = 0;

        // Loop on keyboard input
        while let Ok(Event::Key(KeyEvent { code, .. })) = read() {
            match code {
                KeyCode::Char(c) => {
                    // Add character in expression and move cursor to right
                    if cursor_pos == expression.len() {
                        expression.push(c);
                    } else {
                        expression.insert(cursor_pos, c);
                    }

                    cursor_pos += 1;

                    self.update_current_line(&expression, cursor_pos)?;
                }
                KeyCode::Left => {
                    // Move cursor to left
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        execute!(self.stream, MoveLeft(1))?;
                    }
                }
                KeyCode::Right => {
                    // Move cursor to right
                    if cursor_pos < expression.len() {
                        cursor_pos += 1;
                        execute!(self.stream, MoveRight(1))?;
                    }
                }
                KeyCode::Backspace => {
                    // Remove character from expression and move cursor to left
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        expression.remove(cursor_pos);

                        self.update_current_line(&expression, cursor_pos)?;
                    }
                }
                KeyCode::Up => {
                    // Set expression to previous expression entered by user
                    if let Some(last_expression) = history_iter.next() {
                        expression = last_expression.clone();
                        cursor_pos = expression.len();

                        self.update_current_line(&expression, cursor_pos)?;
                    }
                }
                KeyCode::Esc => {
                    // Set expression to quit to ask at the application to exit
                    expression = String::from("quit");
                    execute!(self.stream, MoveToNextLine(1))?;

                    break;
                }
                KeyCode::Enter => {
                    // Terminate the entry
                    execute!(self.stream, MoveToNextLine(1))?;
                    break;
                }
                _ => continue,
            }
        }

        return Ok(expression);
    }

    // Display text on current line
    fn display_text(&mut self, text: &String) -> Result<(), Error> {
        write!(self.stream, "{}", text)?;
        self.stream.flush()?;

        return Ok(());
    }

    // Display text on current line and go to next line
    fn display_text_with_new_line(&mut self, text: &String) -> Result<(), Error> {
        write!(self.stream, "{}", text)?;
        execute!(self.stream, MoveToNextLine(1))?;
        self.stream.flush()?;

        return Ok(());
    }
}
