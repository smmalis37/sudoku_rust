#![feature(associated_type_bounds)]

mod cell;
mod consts;
mod prelude;
mod solo_state;
mod sudoku;
mod sudoku_array;

use crate::prelude::*;
use iced::*;

fn main() -> Result {
    let window_size = (SIZE2 * 80) as u32;
    <Sudoku as Application>::run(Settings {
        window: window::Settings {
            size: (window_size, window_size),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
