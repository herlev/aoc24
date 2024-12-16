use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::fmt;

use crate::{Direction, DirectionDiag};

#[derive(Add, AddAssign, Sub, SubAssign, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point2<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point2<T> {
  pub fn from(p: (T, T)) -> Self {
    Self { x: p.0, y: p.1 }
  }
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl Point2<i32> {
  pub fn neighbors(self) -> impl Iterator<Item = Point2<i32>> {
    [Direction::Up, Direction::Down, Direction::Right, Direction::Left]
      .into_iter()
      .map(move |d| self + d.to_point())
  }
  pub fn neighbors8(self) -> impl Iterator<Item = Point2<i32>> {
    [
      DirectionDiag::Up,
      DirectionDiag::Upleft,
      DirectionDiag::Upright,
      DirectionDiag::Down,
      DirectionDiag::Downleft,
      DirectionDiag::Downright,
      DirectionDiag::Right,
      DirectionDiag::Left,
    ]
    .into_iter()
    .map(move |d| self + d.to_point())
  }
  pub fn neighbors_grid(self, w: usize, h: usize) -> impl Iterator<Item = Point2<i32>> {
    self
      .neighbors()
      .filter(move |n| (0 as i32..w as i32).contains(&n.x) && (0 as i32..h as i32).contains(&n.y))
  }
}

impl<T: num::traits::Signed> Point2<T> {
  pub fn signum(&self) -> Point2<T> {
    Point2::new(self.x.signum(), self.y.signum())
  }
  pub fn abs(&self) -> Point2<T> {
    Point2::new(self.x.abs(), self.y.abs())
  }
}

impl<T: std::fmt::Display> fmt::Display for Point2<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Point2<T> {
  type Output = Self;

  fn mul(self, scalar: T) -> Self {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }
}

// impl<T> std::ops::Mul<Point2<T>> for T
// where
//   T: std::ops::Mul<Output = T> + Copy,
// {
//   type Output = Point2<T>;

//   fn mul(self, point: Point2<T>) -> Point2<T> {
//     Point2 {
//       x: self * point.x,
//       y: self * point.y,
//     }
//   }
// }
