use crate::prelude::*;
use iced::*;

#[derive(Default)]
pub struct Sudoku {
    _clear: button::State,
    _fill: button::State,
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

    fn view(&mut self) -> Element<'_, M> {
        let mut column = Column::new();

        for y in 0..SIZE2 {
            let mut row = Row::new().height(Length::FillPortion(50));

            for x in 0..SIZE2 {
                if x % SIZE == 0 && x != 0 {
                    row = row.push(Space::with_width(Length::FillPortion(1)));
                }
                row = row.push(Cell::default().width(Length::FillPortion(50)));
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
