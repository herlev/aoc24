use std::ops::RangeInclusive;

use itertools::iproduct;

use crate::Point3;

#[derive(Debug)]
pub struct Grid3<T> {
  data: Vec<Vec<Vec<T>>>,
  pub x_range: RangeInclusive<i32>,
  pub y_range: RangeInclusive<i32>,
  pub z_range: RangeInclusive<i32>,
}

impl<T: Default + Clone> Grid3<T> {
  pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>, z_range: RangeInclusive<i32>) -> Self {
    let xd = (x_range.end() - x_range.start()) as usize;
    let yd = (y_range.end() - y_range.start()) as usize;
    let zd = (z_range.end() - z_range.start()) as usize;
    let g = vec![vec![vec![T::default(); zd + 1]; yd + 1]; xd + 1];
    Self {
      data: g,
      x_range,
      y_range,
      z_range,
    }
  }
  pub fn iter(&self) -> impl Iterator<Item = Point3<i32>> {
    iproduct!(self.x_range.clone(), self.y_range.clone(), self.z_range.clone()).map(Point3::from)
  }
  pub fn at_point(&mut self, p: &Point3<i32>) -> Option<&mut T> {
    self.at(&p.x, &p.y, &p.z)
  }
  pub fn at(&mut self, x: &i32, y: &i32, z: &i32) -> Option<&mut T> {
    if self.x_range.contains(&x) && self.y_range.contains(&y) && self.z_range.contains(&z) {
      let xn = (x - self.x_range.start()) as usize;
      let yn = (y - self.y_range.start()) as usize;
      let zn = (z - self.z_range.start()) as usize;
      return Some(&mut self.data[xn][yn][zn]);
    }
    None
  }
}
