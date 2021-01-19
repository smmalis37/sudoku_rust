use crate::{
    consts::*,
    grid_space::{Cell, GridSpace},
    solo_state::SoloState,
    sudoku_array::SudokuArray,
};
use arraytools::ArrayTools;
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
    let root_id = WidgetId::next();

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
                GridSpace::new(root_id, up_target, down_target)
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
                    .on_click(move |ctx, _g, _env| ctx.submit_command(CLEAR_SELECTOR.to(root_id))),
            )
            .with_child(
                Button::new("Fill in").on_click(move |ctx, _g, _env| {
                    ctx.submit_command(FILL_IN_SELECTOR.to(root_id))
                }),
            ),
    );

    column.controller(Grid).with_id(root_id)
}

struct Grid;

#[allow(clippy::needless_range_loop)]
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
            Event::Command(c) if c.is(REGENERATE_SELECTOR) => {
                println!("Regenerate");

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
                            for square_y in y_corner..y_corner + SIZE {
                                let x_corner = (x / SIZE) * SIZE;
                                for square_x in x_corner..x_corner + SIZE {
                                    data.cells[square_y][square_x].g.possibilities[n] = false;
                                }
                            }

                            data.cells[y][x].g.possibilities = SudokuArray::new(false);
                            data.cells[y][x].g.possibilities[n] = value_possible;
                        }
                    }
                }

                let mut row_solos = <[_; SIZE2]>::generate(|| SudokuArray::new(SoloState::None));
                let mut col_solos = <[_; SIZE2]>::generate(|| SudokuArray::new(SoloState::None));
                let mut square_solos = <[_; SIZE]>::generate(|| {
                    <[_; SIZE]>::generate(|| SudokuArray::new(SoloState::None))
                });

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        for (n, _) in data.cells[y][x].possibility_iter().filter(|&(_, p)| p) {
                            row_solos[y][n].increment((y, x));
                            col_solos[x][n].increment((y, x));
                            square_solos[y / SIZE][x / SIZE][n].increment((y, x));
                        }
                    }
                }

                for group in row_solos
                    .iter()
                    .chain(col_solos.iter())
                    .chain(square_solos.iter().flatten())
                {
                    for (n, &s) in group.enumerate() {
                        if let SoloState::Solo((y, x)) = s {
                            data.cells[y][x].g.solo.increment(n);
                        }
                        // TODO: Make something red on nones
                    }
                }
            }

            Event::Command(c) if c.is(FILL_IN_SELECTOR) => {
                println!("Fill in");
                let mut changed = false;

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        changed |= data.cells[y][x].attempt_fill();
                    }
                }

                if changed {
                    ctx.submit_command(REGENERATE_SELECTOR.to(ctx.widget_id()));
                }
            }

            Event::Command(c) if c.is(CLEAR_SELECTOR) => {
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
