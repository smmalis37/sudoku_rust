#[derive(Copy, Clone)]
pub enum SoloState<T: PartialEq> {
    None,
    Solo(T),
    Multiple,
}

impl<T: PartialEq> SoloState<T> {
    pub fn increment(&mut self, val: T) {
        match self {
            Self::None => *self = Self::Solo(val),
            Self::Solo(x) if *x != val => *self = Self::Multiple,
            _ => {}
        }
    }
}
