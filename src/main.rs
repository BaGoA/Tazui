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

mod application;
mod calculator;
mod crossterm_tui;
mod tui;

use application::Application;
use crossterm_tui::CrosstermTui;

fn main() {
    let mut app: Application<CrosstermTui> = Application::<CrosstermTui>::new();

    if let Err(error) = app.init() {
        println!("{}\n", error);
    }

    match app.run() {
        Ok(()) => (),
        Err(error) => println!("{}\n", error),
    }
}
