use itertools::*;
extern crate nalgebra as na;
use na::*;

#[derive(Debug)]
struct ClawMachine {
  ax: u32,
  ay: u32,
  bx: u32,
  by: u32,
  x: u32,
  y: u32,
}

fn parse_button(s: &str) -> (u32, u32) {
  s.split_once(": ")
    .unwrap()
    .1
    .split(", ")
    .map(|p| p.split_once("+").unwrap().1)
    .map(|s| s.parse().unwrap())
    .collect_tuple()
    .unwrap()
}

fn parse_prize(s: &str) -> (u32, u32) {
  s.split_once(": ")
    .unwrap()
    .1
    .split(", ")
    .map(|p| p.split_once("=").unwrap().1)
    .map(|s| s.parse().unwrap())
    .collect_tuple()
    .unwrap()
}

fn parse(s: &str) -> ClawMachine {
  let mut lines = s.lines();
  let ((ax, ay), (bx, by)): (_, _) = lines.clone().take(2).map(parse_button).collect_tuple().unwrap();
  let (x, y) = parse_prize(lines.nth(2).unwrap());
  ClawMachine { ax, ay, bx, by, x, y }
}

type Matrix2x2f64 = SMatrix<f64, 2, 2>;
type Matrix2x2u64 = SMatrix<u64, 2, 2>;
fn solve(c: ClawMachine, offset: u64) -> Option<u64> {
  let (ax, bx, ay, by, x, y) = (
    c.ax as f64,
    c.bx as f64,
    c.ay as f64,
    c.by as f64,
    c.x as f64,
    c.y as f64,
  );
  let m = Matrix2x2f64::new(ax, bx, ay, by);
  let pos = Vector2::new(x + offset as f64, y + offset as f64);
  let moves = m.try_inverse()? * pos;
  let (a, b) = (moves[0].round() as u64, moves[1].round() as u64);
  let correct_pos = Vector2::new(c.x as u64 + offset, c.y as u64 + offset);
  let new_pos = Matrix2x2u64::new(c.ax as u64, c.bx as u64, c.ay as u64, c.by as u64) * Vector2::new(a, b);
  if new_pos == correct_pos {
    Some(3 * a + b)
  } else {
    None
  }
}

fn solve_part1(c: ClawMachine) -> Option<u64> {
  solve(c, 0)
}

fn solve_part2(c: ClawMachine) -> Option<u64> {
  solve(c, 10000000000000)
}

fn main() {
  let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
  let input = include_str!("input.txt");
  let part1: u64 = input.split("\n\n").map(parse).filter_map(solve_part1).sum();
  let part2: u64 = input.split("\n\n").map(parse).filter_map(solve_part2).sum();
  dbg!(part1);
  dbg!(part2);
}
