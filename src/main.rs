#![feature(associated_type_bounds)]

mod cell;
mod consts;
mod solo_state;
mod sudoku;
mod sudoku_array;

pub(crate) use cell::Cell;
pub(crate) use consts::*;
pub(crate) use solo_state::SoloState;
pub(crate) use sudoku::Sudoku;
pub(crate) use sudoku_array::SudokuArray;

fn main() -> iced::Result {
    let window_size = (SIZE2 * 80) as u32;
    <Sudoku as iced::Application>::run(iced::Settings {
        window: iced::window::Settings {
            size: (window_size, window_size),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
