mod consts;
mod grid;
mod grid_space;

use consts::SIZE2;
use grid::make_grid;

use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(
        WindowDesc::new(make_grid)
            .title("Sudoku")
            .resizable(false)
            .window_size((SIZE2 as f64 * 80.0, SIZE2 as f64 * 80.0)),
    )
    .launch(Default::default())
}
