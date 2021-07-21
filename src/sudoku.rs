use crate::*;
use iced::*;

#[derive(Default)]
pub struct Sudoku {
    cells: [[Cell; SIZE2]; SIZE2],
    // clear: button::State,
    // fill: button::State,
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

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut column = Column::new();

        for (y, row_cells) in self.cells.iter_mut().enumerate() {
            let mut row = Row::new().height(Length::FillPortion(50));

            for (x, cell) in row_cells.iter_mut().enumerate() {
                if x % SIZE == 0 && x != 0 {
                    row = row.push(Space::with_width(Length::FillPortion(1)));
                }
                row = row.push(cell.view().width(Length::FillPortion(50)));
            }

            if y % SIZE == 0 && y != 0 {
                column = column.push(Space::with_height(Length::FillPortion(1)));
            }
            column = column.push(row);
        }

        column.into()
    }

    fn update(&mut self, message: Self::Message) {}
}
