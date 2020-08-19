use druid::{widget::*, *};

mod grid_space;
use grid_space::*;

const SIZE: usize = 3;
const SIZE2: usize = SIZE * SIZE;
const BASE: u8 = SIZE2 as u8 + 1;

const RECOMPUTE_SELECTOR: Selector<()> = Selector::new("recompute");

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(
        WindowDesc::new(main_window)
            .title("Sudoku")
            .resizable(false)
            .window_size((SIZE2 as f64 * 80.0, SIZE2 as f64 * 80.0)),
    )
    .launch(Default::default())
}

#[derive(Clone, Default, Data)]
struct State {
    cells: [[Cell; SIZE2]; SIZE2],
}

fn main_window() -> impl Widget<State> {
    const SPACER_FLEX: f64 = 0.02;
    let mut column = Flex::column();
    let root_id = WidgetId::next();

    for y in 0usize..SIZE2 {
        let mut row = Flex::row();

        for x in 0usize..SIZE2 {
            if x % SIZE == 0 && x != 0 {
                row.add_flex_spacer(SPACER_FLEX);
            }

            row.add_flex_child(
                GridSpace::new(root_id).lens(lens::Field::new(
                    move |g: &State| &g.cells[y][x],
                    move |g| &mut g.cells[y][x],
                )),
                1.0,
            );
        }

        if y % SIZE == 0 && y != 0 {
            column.add_flex_spacer(SPACER_FLEX);
        }

        column.add_flex_child(row, 1.0);
    }

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
                let mut new_possibilities: [[[bool; SIZE2]; SIZE2]; SIZE2] =
                    [[[true; SIZE2]; SIZE2]; SIZE2];

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        if let Some(n) = data.cells[y][x].value {
                            let index = n as usize - 1;
                            for col in 0..SIZE2 {
                                new_possibilities[col][x][index] = false;
                            }
                            for row in 0..SIZE2 {
                                new_possibilities[y][row][index] = false;
                            }
                            let y_corner = (y / SIZE) * SIZE;
                            for square_y in y_corner..y_corner + SIZE {
                                let x_corner = (x / SIZE) * SIZE;
                                for square_x in x_corner..x_corner + SIZE {
                                    new_possibilities[square_y][square_x][index] = false;
                                }
                            }
                        }
                    }
                }

                for y in 0..SIZE2 {
                    for x in 0..SIZE2 {
                        data.cells[y][x].possibilities = new_possibilities[y][x];
                    }
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
