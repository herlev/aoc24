use std::collections::HashMap;

use itertools::*;

fn get_next(n: i64) -> i64 {
  let n = (n * 64 ^ n).rem_euclid(16777216);
  let n = (n / 32 ^ n).rem_euclid(16777216);
  let n = (n * 2048 ^ n).rem_euclid(16777216);
  n
}

struct Rng {
  curr: i64,
}

impl Rng {
  fn new(n: i64) -> Self {
    Self { curr: n }
  }
}

impl Iterator for Rng {
  type Item = i64;

  fn next(&mut self) -> Option<Self::Item> {
    let c = self.curr;
    self.curr = get_next(c);
    Some(c)
  }
}

fn part1(input: &str) {
  let rngs = input.trim().split("\n").map(|s| s.parse().unwrap()).map(Rng::new);

  let part1: i64 = rngs.map(|rng| rng.take(2001).last().unwrap()).sum();
  dbg!(part1);
}

type MyMap = HashMap<[i64; 4], i64>;

fn map(values: &[(i64, i64)]) -> MyMap {
  let mut result = HashMap::new();
  values.into_iter().tuple_windows().for_each(|(a, b, c, d)| {
    let a = [a.1, b.1, c.1, d.1];
    let diff = d.0;
    if !result.contains_key(&a) {
      result.insert(a, diff);
    }
  });
  result
}

fn merge(mut a: MyMap, b: MyMap) -> MyMap {
  for (key, value) in b {
    *a.entry(key).or_default() += value;
  }
  a
}

fn part2(input: &str) {
  let merged = input
    .trim()
    .lines()
    .map(|s| s.parse().unwrap())
    .map(Rng::new)
    .map(|rng| {
      rng
        .take(2001)
        .tuple_windows()
        .map(|(a, b)| (b % 10, b % 10 - a % 10))
        .collect_vec()
    })
    .map(|v| map(&v))
    .reduce(merge)
    .unwrap();
  let part2 = merged.values().max().unwrap();
  dbg!(part2);
}

fn main() {
  let input = "1
10
100
2024";
  let input = "1
2
3
2024";
  let input = include_str!("input.txt");
  part1(input);
  part2(input);
}
