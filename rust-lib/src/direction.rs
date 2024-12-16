use crate::Point2;

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[derive(Clone, Debug, Copy)]
pub enum DirectionDiag {
  Upleft,
  Up,
  Upright,
  Right,
  Downright,
  Down,
  Downleft,
  Left,
}

impl DirectionDiag {
  pub fn to_point(&self) -> Point2<i32> {
    match self {
      Self::Up => Point2::new(0, 1),
      Self::Down => Point2::new(0, -1),
      Self::Right => Point2::new(1, 0),
      Self::Left => Point2::new(-1, 0),
      Self::Upleft => Point2::new(-1, 1),
      Self::Upright => Point2::new(1, 1),
      Self::Downleft => Point2::new(-1, -1),
      Self::Downright => Point2::new(1, -1),
    }
  }
}

impl Direction {
  pub fn all() -> impl Iterator<Item = Direction> {
    [Direction::Up, Direction::Down, Direction::Right, Direction::Left].into_iter()
  }
  pub fn to_point(&self) -> Point2<i32> {
    match self {
      Self::Up => Point2::new(0, 1),
      Self::Down => Point2::new(0, -1),
      Self::Right => Point2::new(1, 0),
      Self::Left => Point2::new(-1, 0),
    }
  }
  pub fn opposite(&self) -> Self {
    match self {
      Self::Up => Self::Down,
      Self::Down => Self::Up,
      Self::Right => Self::Left,
      Self::Left => Self::Right,
    }
  }
  pub fn cw(&self) -> Self {
    match self {
      Self::Up => Self::Right,
      Self::Down => Self::Left,
      Self::Right => Self::Down,
      Self::Left => Self::Up,
    }
  }
  pub fn ccw(&self) -> Self {
    match self {
      Self::Up => Self::Left,
      Self::Down => Self::Right,
      Self::Right => Self::Up,
      Self::Left => Self::Down,
    }
  }
  pub fn from_point(p: Point2<i32>) -> Self {
    match p.signum() {
      Point2 { x: 0, y: 1 } => Self::Up,
      Point2 { x: 0, y: -1 } => Self::Down,
      Point2 { x: 1, y: 0 } => Self::Right,
      Point2 { x: -1, y: 0 } => Self::Left,
      _ => panic!(),
    }
  }
}
