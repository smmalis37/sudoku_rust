use crate::consts::*;
use std::ops::{Index, IndexMut};

pub struct SudokuArray<T>([T; SIZE2]);

impl<T: Copy> SudokuArray<T> {
    pub fn new(val: T) -> Self {
        Self([val; SIZE2])
    }
}

impl<T> SudokuArray<T> {
    pub fn enumerate(&self) -> impl Iterator<Item = (Num, &T)> {
        (1..).zip(self.0.iter())
    }
}

impl<T> Index<Num> for SudokuArray<T> {
    type Output = T;

    fn index(&self, index: Num) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> Index<usize> for SudokuArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}

impl<T> IndexMut<Num> for SudokuArray<T> {
    fn index_mut(&mut self, index: Num) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> IndexMut<usize> for SudokuArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}
