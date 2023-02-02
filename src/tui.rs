/*
   Tazui is simple terminal calculator
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

/// Trait to define terminal user interface functionnalities
/// that Tazui application needs.
pub trait Tui {
    fn init(&mut self) -> Result<(), std::io::Error>;
    fn get_start_of_line(&self) -> String;
    fn get_expression(&mut self, history: &Vec<String>) -> Result<String, std::io::Error>;
    fn display_text(&mut self, text: &String) -> Result<(), std::io::Error>;
    fn display_text_with_new_line(&mut self, text: &String) -> Result<(), std::io::Error>;
}
