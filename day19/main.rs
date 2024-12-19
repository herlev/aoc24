use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn dfs(s: &str, patterns: &HashSet<&str>) -> bool {
  if s.is_empty() {
    return true;
  }
  for i in (0..=s.len()).rev() {
    if patterns.contains(&s[0..i]) && dfs(&s[i..], patterns) {
      return true;
    }
  }
  false
}

fn dfs2<'a>(s: &'a str, patterns: &HashSet<&str>, cache: &mut HashMap<&'a str, u64>) -> u64 {
  if s.is_empty() {
    return 1;
  }
  if let Some(&v) = cache.get(s) {
    return v;
  }
  let mut sum = 0;
  for i in 0..=s.len() {
    if patterns.contains(&s[0..i]) {
      sum += dfs2(&s[i..], patterns, cache);
    }
  }
  cache.insert(s, sum);
  sum
}

fn main() {
  let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
  let input = include_str!("input.txt");
  let (patterns, designs) = input.trim().split_once("\n\n").unwrap();
  let patterns: HashSet<&str> = patterns.split(", ").collect();
  let designs = designs.split("\n").collect_vec();

  let part1 = designs.iter().map(|d| dfs(d, &patterns)).filter(|&b| b).count();
  let part2: u64 = designs.iter().map(|d| dfs2(d, &patterns, &mut HashMap::new())).sum();
  dbg!(part1);
  dbg!(part2);
}
