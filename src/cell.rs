use crate::*;
use iced::*;

pub struct Cell {
    // User controlled data
    value: Option<Num>,
    user_removed: SudokuArray<bool>,

    // Generated data
    possibilities: SudokuArray<bool>,
    //solo: SoloState<Num>,
    //in_invalid_group: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            user_removed: SudokuArray::new(false),
            possibilities: SudokuArray::new(true),
        }
    }
}

impl Cell {
    pub fn view(&self) -> Container<'_, <Sudoku as Application>::Message> {
        let content: iced::Element<'_, _> = match self.value {
            Some(_) => self.make_value_text().into(),
            None => self.make_possibility_grid().into(),
        };

        Container::new(content).center_x().center_y().style(Theme)
    }

    fn make_value_text(&self) -> Text {
        Text::new(self.value.unwrap().to_string())
            .size(48)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
    }

    fn make_possibility_grid(&self) -> Column<'_, <Sudoku as Application>::Message> {
        let mut column = Column::new().align_items(Align::Center);

        for y in 0..SIZE {
            let mut row = Row::new().height(Length::Fill).align_items(Align::Center);

            for x in 0..SIZE {
                let num = y * SIZE + x + 1;
                let s = if !self.possibilities[num] {
                    String::new()
                } else if self.user_removed[num] {
                    "█".to_string()
                } else {
                    radix_string(num)
                };

                row = row.push(
                    Text::new(s)
                        .size(14)
                        .color([0.5, 0.5, 0.5])
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                );
            }

            column = column.push(row);
        }

        column
    }
}

struct Theme;

impl container::StyleSheet for Theme {
    fn style(&self) -> container::Style {
        container::Style {
            border_width: 1.0,
            border_color: [0.75, 0.75, 0.75].into(),
            background: Some(Background::Color(Color::WHITE)),
            ..Default::default()
        }
    }
}

fn radix_string<T>(n: T) -> String
where
    radix_fmt::Radix<T>: std::fmt::Display,
{
    format!("{:#}", radix_fmt::radix(n, BASE))
}
