use derive_more::{Add, AddAssign, Sub, SubAssign};
#[derive(Add, AddAssign, Sub, SubAssign, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Point3<T> {
  pub fn from(p: (T, T, T)) -> Self {
    Self { x: p.0, y: p.1, z: p.2 }
  }
  pub fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }
}

impl Point3<i32> {
  pub fn neighbors(&self) -> [Point3<i32>; 6] {
    let rr = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
    rr.map(|p| Point3::from(p)).map(|p| *self + p)
  }
}
