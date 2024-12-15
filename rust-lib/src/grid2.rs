use std::ops::RangeInclusive;

use itertools::{iproduct, Itertools};

use crate::Point2;

#[derive(Debug)]
pub struct Grid2<T> {
  data: Vec<Vec<T>>, // col<row<data>>
  pub x_range: RangeInclusive<i32>,
  pub y_range: RangeInclusive<i32>,
}

impl<T: Default + Clone> Grid2<T> {
  pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
    let xd = (x_range.end() - x_range.start()) as usize;
    let yd = (y_range.end() - y_range.start()) as usize;
    let g = vec![vec![T::default(); xd + 1]; yd + 1];
    Self {
      data: g,
      x_range,
      y_range,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = Point2<i32>> {
    iproduct!(self.x_range.clone(), self.y_range.clone()).map(Point2::from)
  }

  pub fn at_point(&self, p: &Point2<i32>) -> Option<&T> {
    self.at(p.x, p.y)
  }

  pub fn at_point_mut(&mut self, p: &Point2<i32>) -> Option<&mut T> {
    self.at_mut(p.x, p.y)
  }

  pub fn at(&self, x: i32, y: i32) -> Option<&T> {
    if self.x_range.contains(&x) && self.y_range.contains(&y) {
      let xn = (x - self.x_range.start()) as usize;
      let yn = (y - self.y_range.start()) as usize;
      return Some(&self.data[yn][xn]);
    }
    None
  }

  pub fn at_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
    if self.x_range.contains(&x) && self.y_range.contains(&y) {
      let xn = (x - self.x_range.start()) as usize;
      let yn = (y - self.y_range.start()) as usize;
      return Some(&mut self.data[yn][xn]);
    }
    None
  }
  pub fn from_char_grid(input: &str, transformer: &mut dyn FnMut(Point2<i32>, char) -> T) -> Self {
    let data = input
      .lines()
      .rev()
      .enumerate()
      .map(|(y, line)| {
        line
          .chars()
          .enumerate()
          .map(|(x, c)| {
            transformer(
              Point2 {
                x: x as i32,
                y: y as i32,
              },
              c,
            )
          })
          .collect_vec()
      })
      .collect_vec();
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    assert!(data.iter().all(|v| v.len() as i32 == width));
    Self {
      data,
      x_range: 0..=width - 1,
      y_range: 0..=height - 1,
    }
  }
  pub fn print(&self, transformer: &dyn Fn(Point2<i32>, &T) -> char) {
    for y in self.y_range.clone().rev() {
      for x in self.x_range.clone() {
        let p = Point2::new(x, y);
        print!("{}", transformer(p, self.at_point(&p).unwrap()));
      }
      println!("");
    }
  }
}
