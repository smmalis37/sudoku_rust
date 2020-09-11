use druid::Data;

#[derive(Copy, Clone, Data)]
pub enum SoloState<T: PartialEq> {
    None,
    Solo(T),
    Multiple,
}

impl<T: PartialEq> SoloState<T> {
    pub fn increment(&mut self, val: T) {
        match self {
            SoloState::None => *self = SoloState::Solo(val),
            SoloState::Solo(x) if *x != val => *self = SoloState::Multiple,
            _ => {}
        }
    }
}
