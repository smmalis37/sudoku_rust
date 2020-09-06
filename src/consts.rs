use druid::Selector;

pub type Num = u8;

pub const SIZE: usize = 3;
pub const SIZE2: usize = SIZE * SIZE;
pub const BASE: u8 = SIZE2 as u8 + 1;

pub const RECOMPUTE_SELECTOR: Selector<()> = Selector::new("recompute");
pub const FILL_IN_SELECTOR: Selector<()> = Selector::new("fill-in");
