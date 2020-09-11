use crate::consts::*;
use druid::Data;
use std::ops::{Index, IndexMut};

// TODO: Remove Copy once we can drain an array
#[derive(Copy, Clone, Data)]
pub struct SudokuArray<T: Copy + Clone + Data>([T; SIZE2]);

impl<T: Copy + Clone + Data> SudokuArray<T> {
    pub fn new(val: T) -> Self {
        Self([val; SIZE2])
    }
}

impl<T: Copy + Clone + Data> Index<Num> for SudokuArray<T> {
    type Output = T;

    fn index(&self, index: Num) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T: Copy + Clone + Data> Index<usize> for SudokuArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}

impl<T: Copy + Clone + Data> IndexMut<Num> for SudokuArray<T> {
    fn index_mut(&mut self, index: Num) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T: Copy + Clone + Data> IndexMut<usize> for SudokuArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}

// TODO remove to prevent misuses, like enumerate
impl<T: Copy + Clone + Data> std::ops::Deref for SudokuArray<T> {
    type Target = [T; SIZE2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
