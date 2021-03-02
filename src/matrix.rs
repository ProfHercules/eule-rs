use std::{
  ops::{Index, IndexMut},
  slice::Iter,
};

#[derive(Debug)]
pub struct Matrix<T> {
  data: Vec<T>,
  width: usize,
  height: usize,
}

impl<T> Matrix<T> {
  pub fn new(width: usize, v: Vec<T>) -> Self {
    assert_eq!(v.len() % width, 0);
    let height = v.len() / width;

    Matrix { data: v, width, height }
  }

  fn get_idx(&self, x: usize, y: usize) -> usize {
    x + (self.width * y)
  }

  fn get_row(&self, idx: usize) -> &[T] {
    let start = idx * self.width;
    &self.data[start..start + self.width]
  }

  fn get_col(&self, idx: usize) -> &[T] {
    self.data[idx..].into_iter().step_by(self.width).collect()
  }

  fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
    self.data.chunks_exact(self.width)
  }

  fn iter_cols(&self) -> impl Iterator<Item = Vec<&T>> {
    self.data.chunks_exact(self.width).map(|c| {})
  }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
  type Output = T;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let (x, y) = index;
    let idx = self.get_idx(x, y);
    &self.data[idx]
  }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let (x, y) = index;
    let idx = self.get_idx(x, y);
    &mut self.data[idx]
  }
}

pub trait IntoMatrix<T> {
  fn into_matrix(self, width: usize) -> Matrix<T>;
}

impl<T> IntoMatrix<T> for Vec<T> {
  fn into_matrix(self, width: usize) -> Matrix<T> {
    Matrix::new(width, self)
  }
}
