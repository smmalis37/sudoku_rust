use crate::consts::*;
use druid::Data;
use std::ops::{Index, IndexMut};

#[derive(Clone, Data)]
pub struct SudokuArray<T: Clone + Data>([T; SIZE2]);

impl<T: Copy + Clone + Data> SudokuArray<T> {
    pub fn new(val: T) -> Self {
        Self([val; SIZE2])
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Num, &T)> {
        (1..).zip(self.0.iter())
    }
}

impl<T: Clone + Data> Index<Num> for SudokuArray<T> {
    type Output = T;

    fn index(&self, index: Num) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T: Clone + Data> Index<usize> for SudokuArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}

impl<T: Clone + Data> IndexMut<Num> for SudokuArray<T> {
    fn index_mut(&mut self, index: Num) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T: Clone + Data> IndexMut<usize> for SudokuArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}
