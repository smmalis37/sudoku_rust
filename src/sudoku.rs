use crate::cell::*;
use crate::prelude::*;
use arraytools::ArrayTools;
use arrayvec::ArrayVec;
use iced::*;

#[derive(Default)]
pub(crate) struct Sudoku {
    cells: [[State; SIZE2]; SIZE2],
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

        for (y, states) in self.cells.iter_mut().enumerate() {
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
            Regen => self.regen(),

            Fill => {
                let mut changed = false;
                for s in self.cells.iter_mut().flatten() {
                    changed |= s.attempt_fill();
                }
                if changed {
                    Sandbox::update(self, Regen);
                }
            }

            Clear => {
                for s in self.cells.iter_mut().flatten() {
                    *s = Default::default();
                }
            }

            Redraw => {}
        }
    }
}

impl Sudoku {
    #[allow(clippy::needless_range_loop)]
    fn regen(&mut self) {
        let start = std::time::Instant::now();
        print!("Regenerate ");

        for y in 0..SIZE2 {
            for x in 0..SIZE2 {
                self.cells[y][x].g = Default::default();
            }
        }

        for y in 0..SIZE2 {
            for x in 0..SIZE2 {
                if let Some(n) = self.cells[y][x].value() {
                    let value_possible = self.cells[y][x].g.possibilities[n];

                    for col in 0..SIZE2 {
                        self.cells[col][x].g.possibilities[n] = false;
                    }
                    for row in 0..SIZE2 {
                        self.cells[y][row].g.possibilities[n] = false;
                    }
                    let y_corner = (y / SIZE) * SIZE;
                    let x_corner = (x / SIZE) * SIZE;
                    for square_y in y_corner..y_corner + SIZE {
                        for square_x in x_corner..x_corner + SIZE {
                            self.cells[square_y][square_x].g.possibilities[n] = false;
                        }
                    }

                    self.cells[y][x].g.possibilities = SudokuArray::new(false);
                    self.cells[y][x].g.possibilities[n] = value_possible;
                }
            }
        }

        let mut square_solos = <[_; SIZE]>::generate(|| {
            <[_; SIZE]>::generate(|| {
                (
                    SudokuArray::new(SoloState::None),
                    ArrayVec::<(usize, usize), SIZE2>::new(),
                )
            })
        });
        let mut row_solos =
            <[_; SIZE2]>::generate(|| (SudokuArray::new(SoloState::None), ArrayVec::new()));
        let mut col_solos = row_solos.clone();

        for y in 0..SIZE2 {
            for x in 0..SIZE2 {
                //TODO: don't compute group cells every time, they don't change
                let pos = (y, x);
                row_solos[y].1.push(pos);
                col_solos[x].1.push(pos);
                square_solos[y / SIZE][x / SIZE].1.push(pos);
                for n in self.cells[y][x].possibility_iter() {
                    row_solos[y].0[n].increment(pos);
                    col_solos[x].0[n].increment(pos);
                    square_solos[y / SIZE][x / SIZE].0[n].increment(pos);
                }
            }
        }

        for (group, cells) in row_solos
            .iter()
            .chain(col_solos.iter())
            .chain(square_solos.iter().flatten())
        {
            for (n, &s) in group.enumerate() {
                match s {
                    SoloState::Solo((y, x)) => self.cells[y][x].g.solo.increment(n),
                    SoloState::None => {
                        if !cells
                            .iter()
                            .any(|&(y, x)| self.cells[y][x].value() == Some(n))
                        {
                            for &(y, x) in cells {
                                self.cells[y][x].g.in_invalid_group = true;
                            }
                        }
                    }
                    SoloState::Multiple => {}
                }
            }
        }

        let end = std::time::Instant::now();
        println!("{:?}", end - start);
    }
}
