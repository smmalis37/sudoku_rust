use crate::cell::*;
use crate::prelude::*;
use iced::*;

#[derive(Default)]
pub(crate) struct Sudoku {
    states: [[State; SIZE2]; SIZE2],
    clear: button::State,
    fill: button::State,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Message {
    Regen,
    Fill,
    Clear,
    Redraw,
}

impl Sandbox for Sudoku {
    type Message = Message;

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

                row = row.push(Cell::new(state, Length::FillPortion(50)));
            }

            if y % SIZE == 0 && y != 0 {
                column = column.push(Space::with_height(Length::FillPortion(1)));
            }

            column = column.push(row);
        }

        let button_row = Row::new()
            .height(Length::FillPortion(30))
            .width(Length::Fill);

        let clear_button = Button::new(
            &mut self.clear,
            Text::new("Clear")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .on_press(Clear);

        let fill_button = Button::new(
            &mut self.fill,
            Text::new("Fill")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .on_press(Fill);

        column = column.push(button_row.push(clear_button).push(fill_button));

        column.into()
    }

    fn update(&mut self, message: M) {
        match message {
            Regen => todo!(),
            Fill => todo!(),
            Clear => todo!(),
            Redraw => {}
        }
    }
}
