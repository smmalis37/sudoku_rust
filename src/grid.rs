use crate::{consts::*, grid_space::*, index_minus_one::*};
use arraytools::ArrayTools;
use druid::{widget::*, *};

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
    let mut button_row = Flex::row();
    button_row.add_child(
        Button::new("Fill in")
            .on_click(move |ctx, _g, _env| ctx.submit_command(FILL_IN_SELECTOR.to(root_id))),
    );
    column.add_child(button_row);

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
            Event::Command(c) if c.is(RECOMPUTE_SELECTOR) => {
                println!("Recompute");
                let mut new_possibilities = [[IndexMinusOne::new(true); SIZE2]; SIZE2];

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        if let Some(n) = data.cells[y][x].value {
                            let value_possible = new_possibilities[y][x][n];

                            for col in 0..SIZE2 {
                                new_possibilities[col][x][n] = false;
                            }
                            for row in 0..SIZE2 {
                                new_possibilities[y][row][n] = false;
                            }
                            let y_corner = (y / SIZE) * SIZE;
                            for square_y in y_corner..y_corner + SIZE {
                                let x_corner = (x / SIZE) * SIZE;
                                for square_x in x_corner..x_corner + SIZE {
                                    new_possibilities[square_y][square_x][n] = false;
                                }
                            }

                            new_possibilities[y][x][n] = value_possible;
                        }
                    }
                }

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        data.cells[y][x].possibilities = new_possibilities[y][x];
                    }
                }
            }

            Event::Command(c) if c.is(FILL_IN_SELECTOR) => {
                println!("Fill in");
                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        let c = &mut data.cells[y][x];
                        if c.value.is_none() {
                            if let Some(n) = c.one_possibility() {
                                c.value = Some(n);
                            }
                        }
                    }
                }
                ctx.submit_command(RECOMPUTE_SELECTOR.to(ctx.widget_id()));
            }

            _ => child.event(ctx, event, data, env),
        }
    }
}
