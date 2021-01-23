use crate::{
    consts::*,
    grid_space::{Cell, GridSpace},
    solo_state::SoloState,
    sudoku_array::SudokuArray,
};
use arraytools::ArrayTools;
use arrayvec::ArrayVec;
use druid::{
    widget::{Button, Controller, Flex, Widget, WidgetExt, WidgetId},
    Data, Env, Event, EventCtx, Lens, LensExt,
};

#[derive(Clone, Default, Data, Lens)]
pub struct State {
    cells: [[Cell; SIZE2]; SIZE2],
}

pub fn make_grid() -> impl Widget<State> {
    const SPACER_FLEX: f64 = 0.02;
    let mut column = Flex::column();

    let space_ids = <[_; SIZE2]>::generate(|| <[_; SIZE2]>::generate(WidgetId::next));

    for y in 0..SIZE2 {
        let mut row = Flex::row();

        for x in 0..SIZE2 {
            if x % SIZE == 0 && x != 0 {
                row.add_flex_spacer(SPACER_FLEX);
            }

            let up_target = space_ids[if y == 0 { SIZE2 - 1 } else { y - 1 }][x];
            let down_target = space_ids[if y == SIZE2 - 1 { 0 } else { y + 1 }][x];

            row.add_flex_child(
                GridSpace::new(up_target, down_target)
                    .with_id(space_ids[y][x])
                    .lens(State::cells.as_ref().index(y).as_ref().index(x)),
                1.0,
            );
        }

        if y % SIZE == 0 && y != 0 {
            column.add_flex_spacer(SPACER_FLEX);
        }

        column.add_flex_child(row, 1.0);
    }

    column.add_spacer(1.0);

    column.add_child(
        Flex::row()
            .with_child(
                Button::new("Clear")
                    .on_click(move |ctx, _g, _env| ctx.submit_notification(CLEAR_SELECTOR)),
            )
            .with_child(
                Button::new("Fill in")
                    .on_click(move |ctx, _g, _env| ctx.submit_notification(FILL_IN_SELECTOR)),
            ),
    );

    column.controller(Grid)
}

struct Grid;

impl Grid {
    #[allow(clippy::needless_range_loop)]
    fn regenerate(&mut self, data: &mut State) {
        let start = std::time::Instant::now();
        print!("Regenerate ");

        for y in 0..SIZE2 {
            for x in 0..SIZE2 {
                data.cells[y][x].g = Default::default();
            }
        }

        for y in 0..SIZE2 {
            for x in 0..SIZE2 {
                if let Some(n) = data.cells[y][x].value() {
                    let value_possible = data.cells[y][x].g.possibilities[n];

                    for col in 0..SIZE2 {
                        data.cells[col][x].g.possibilities[n] = false;
                    }
                    for row in 0..SIZE2 {
                        data.cells[y][row].g.possibilities[n] = false;
                    }
                    let y_corner = (y / SIZE) * SIZE;
                    let x_corner = (x / SIZE) * SIZE;
                    for square_y in y_corner..y_corner + SIZE {
                        for square_x in x_corner..x_corner + SIZE {
                            data.cells[square_y][square_x].g.possibilities[n] = false;
                        }
                    }

                    data.cells[y][x].g.possibilities = SudokuArray::new(false);
                    data.cells[y][x].g.possibilities[n] = value_possible;
                }
            }
        }

        let mut square_solos = <[_; SIZE]>::generate(|| {
            <[_; SIZE]>::generate(|| {
                (
                    SudokuArray::new(SoloState::None),
                    ArrayVec::<[(usize, usize); SIZE2]>::new(),
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
                for (n, _) in data.cells[y][x].possibility_iter().filter(|&(_, p)| p) {
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
                    SoloState::Solo((y, x)) => data.cells[y][x].g.solo.increment(n),
                    SoloState::None => {
                        if !cells
                            .iter()
                            .any(|&(y, x)| data.cells[y][x].value() == Some(n))
                        {
                            for &(y, x) in cells {
                                data.cells[y][x].g.in_invalid_group = true;
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

impl<W: Widget<State>> Controller<State, W> for Grid {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut State,
        env: &Env,
    ) {
        match event {
            Event::Notification(c) if c.is(REGENERATE_SELECTOR) => self.regenerate(data),

            Event::Notification(c) if c.is(FILL_IN_SELECTOR) => {
                println!("Fill in");
                let mut changed = false;

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        changed |= data.cells[y][x].attempt_fill();
                    }
                }

                if changed {
                    self.regenerate(data)
                }
            }

            Event::Notification(c) if c.is(CLEAR_SELECTOR) => {
                println!("Clear");
                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        data.cells[y][x] = Default::default();
                    }
                }
            }

            _ => child.event(ctx, event, data, env),
        }
    }
}
