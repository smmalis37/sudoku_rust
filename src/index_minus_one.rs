use crate::consts::*;
use druid::*;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Data)]
pub struct IndexMinusOne<T: Copy + Clone + Data>([T; SIZE2]);

impl<T: Copy + Clone + Data> IndexMinusOne<T> {
    pub fn new(val: T) -> Self {
        Self([val; SIZE2])
    }
}

impl<T: Copy + Clone + Data> Index<Num> for IndexMinusOne<T> {
    type Output = T;

    fn index(&self, index: Num) -> &Self::Output {
        &self.0[index as usize - 1]
    }
}

impl<T: Copy + Clone + Data> Index<usize> for IndexMinusOne<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}

impl<T: Copy + Clone + Data> IndexMut<Num> for IndexMinusOne<T> {
    fn index_mut(&mut self, index: Num) -> &mut Self::Output {
        &mut self.0[index as usize - 1]
    }
}

impl<T: Copy + Clone + Data> IndexMut<usize> for IndexMinusOne<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}

// TODO remove to prevent misuses, like enumerate
impl<T: Copy + Clone + Data> std::ops::Deref for IndexMinusOne<T> {
    type Target = [T; SIZE2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
