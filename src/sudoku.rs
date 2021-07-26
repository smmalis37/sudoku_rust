use crate::cell::*;
use crate::prelude::*;
use iced::*;

#[derive(Default)]
pub(crate) struct Sudoku {
    states: [[State; SIZE2]; SIZE2],
    clear: button::State,
    fill: button::State,
}

impl Sandbox for Sudoku {
    type Message = ();

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        "Sudoku".into()
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }

    fn view(&mut self) -> Element<M> {
        let mut column = Column::new();

        for (y, states) in self.states.iter_mut().enumerate() {
            let mut row = Row::new().height(Length::FillPortion(50));

            for (x, state) in states.iter_mut().enumerate() {
                if x % SIZE == 0 && x != 0 {
                    row = row.push(Space::with_width(Length::FillPortion(1)));
                }

                row = row.push(Cell::new(
                    state,
                    Default::default(),
                    Length::FillPortion(50),
                ));
            }

            if y % SIZE == 0 && y != 0 {
                column = column.push(Space::with_height(Length::FillPortion(1)));
            }

            column = column.push(row);
        }

        column.into()
    }

    fn update(&mut self, _message: M) {}
}
