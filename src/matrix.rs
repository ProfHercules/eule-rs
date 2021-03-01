use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T> {
  data: Vec<T>,
  width: usize,
  height: usize,
}

impl<T: Default + Clone> Matrix<T> {
  pub fn from_vec(width: usize, v: Vec<T>) -> Self {
    assert_eq!(v.len() % width, 0);
    let height = v.len() / width;
    Matrix { data: v, width, height }
  }

  fn get_idx(&self, x: usize, y: usize) -> usize {
    x + (self.width * y)
  }
}

impl<T: Default + Clone> Index<(usize, usize)> for Matrix<T> {
  type Output = T;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let (x, y) = index;
    let idx = self.get_idx(x, y);
    &self.data[idx]
  }
}

impl<T: Default + Clone> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let (x, y) = index;
    let idx = self.get_idx(x, y);
    &mut self.data[idx]
  }
}
