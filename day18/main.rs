use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use rust_lib::{Grid2, Point2};

fn get_shortest_path_count(grid: &Grid2<bool>, start: Point2<i32>, end: Point2<i32>) -> Option<u32> {
  let mut visited = HashSet::new();
  let mut q = VecDeque::new();
  q.push_back((start, 0));
  while let Some((p, len)) = q.pop_front() {
    if p == end {
      return Some(len);
    }
    for n in p.neighbors().filter(|n| matches!(grid.at_point(n), Some(false))) {
      if visited.contains(&n) {
        continue;
      }
      q.push_back((n, len + 1));
      visited.insert(n);
    }
  }
  None
}

fn part1(input: &[Point2<i32>], size: i32, num_bytes: usize) {
  let mut grid = Grid2::<bool>::new(0..=size, 0..=size);
  let start = Point2::new(0, 0);
  let end = Point2::new(size, size);
  for p in input.iter().take(num_bytes) {
    *grid.at_point_mut(p).unwrap() = true;
  }
  dbg!(get_shortest_path_count(&grid, start, end));
}

fn part2(input: &[Point2<i32>], size: i32) {
  let mut grid = Grid2::<bool>::new(0..=size, 0..=size);
  let start = Point2::new(0, 0);
  let end = Point2::new(size, size);
  // we could also have used a binary search here instead of brute forcing,
  // but brute force is fast enough, at least when compiled with release mode
  for p in input.iter() {
    *grid.at_point_mut(p).unwrap() = true;
    let a = get_shortest_path_count(&grid, start, end);
    if a.is_none() {
      dbg!(p);
      break;
    }
  }
}

fn main() {
  // let input = include_str!("testinput.txt");
  // let size = 6;
  // let num_bytes = 12;
  let input = include_str!("input.txt");
  let size = 70;
  let num_bytes = 1024;
  dbg!(input.len());

  let input = input
    .lines()
    .map(|line| line.split(",").flat_map(str::parse::<i32>).collect_tuple().unwrap())
    .map(|(x, y)| Point2::new(x, y))
    .collect_vec();

  part1(&input, size, num_bytes);
  part2(&input, size);
}
