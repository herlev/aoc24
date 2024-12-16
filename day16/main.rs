use std::{
  cmp::min,
  collections::{HashMap, HashSet},
};

use rust_lib::{priority_queue::PriorityQueue, Direction, Grid2, Point2};

#[derive(Clone, Debug, Default)]
enum MapElem {
  #[default]
  None,
  Wall,
}

type Map = Grid2<MapElem>;
type Pos = Point2<i32>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
  score: u32,
  pos: Pos,
  dir: Direction,
}

fn print_map(map: &Map, start: Pos, end: Pos) {
  map.print(&|p, e| match e {
    _ if p == start => 'S',
    _ if p == end => 'E',
    MapElem::None => '.',
    MapElem::Wall => '#',
  });
}

fn part1(input: &str) {
  let mut start = None;
  let mut end = None;

  let map = Map::from_char_grid(input, &mut |p, c| match c {
    '.' => MapElem::None,
    '#' => MapElem::Wall,
    'S' => {
      start = Some(p);
      MapElem::None
    }
    'E' => {
      end = Some(p);
      MapElem::None
    }
    _ => todo!("{c}"),
  });
  let start = start.unwrap();
  let end = end.unwrap();
  let mut pq = PriorityQueue::new();
  let mut visited: HashSet<(Direction, Pos)> = HashSet::new();
  let pqe = |state: State| (state.score as usize, state);
  visited.insert((Direction::Right, start));
  pq.push(pqe(State {
    score: 0,
    pos: start,
    dir: Direction::Right,
  }));
  while let Some(State { score, pos, dir }) = pq.pop() {
    if pos == end {
      dbg!(score);
      break;
    }

    for new_dir in Direction::all() {
      let cost = if dir == new_dir { 1 } else { 1001 };
      let next_pos = pos + new_dir.to_point();
      if matches!(map.at_point(&next_pos), Some(MapElem::None)) && !visited.contains(&(new_dir, next_pos)) {
        pq.push(pqe(State {
          score: score + cost,
          pos: next_pos,
          dir: new_dir,
        }));
        visited.insert((new_dir, next_pos));
      }
    }
  }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State2 {
  score: u32,
  pos: Pos,
  dir: Direction,
  path: Vec<Pos>,
}

fn part2(input: &str) {
  let mut start = None;
  let mut end = None;

  let map = Map::from_char_grid(input, &mut |p, c| match c {
    '.' => MapElem::None,
    '#' => MapElem::Wall,
    'S' => {
      start = Some(p);
      MapElem::None
    }
    'E' => {
      end = Some(p);
      MapElem::None
    }
    _ => todo!("{c}"),
  });
  let start = start.unwrap();
  let end = end.unwrap();
  let mut pq = PriorityQueue::new();
  let mut visited: HashSet<(Direction, Pos)> = HashSet::new();
  let pqe = |state: State2| (state.score as usize, state);
  visited.insert((Direction::Right, start));
  pq.push(pqe(State2 {
    score: 0,
    pos: start,
    dir: Direction::Right,
    path: vec![start],
  }));
  let mut best_tiles = HashSet::new();
  let mut best_score = 9999999;
  let mut best_scores = HashMap::<(Direction, Pos), u32>::new();
  while let Some(State2 { score, pos, dir, path }) = pq.pop() {
    dbg!(pq.len());
    if score > best_score {
      break;
    }
    if pos == end {
      best_score = min(best_score, score);
      best_tiles.extend(path);
      continue;
    }

    for new_dir in Direction::all() {
      let cost = if dir == new_dir { 1 } else { 1001 };
      let new_score = score + cost;
      let next_pos = pos + new_dir.to_point();

      let best_score = best_scores.entry((new_dir, next_pos)).or_insert(new_score);
      *best_score = min(*best_score, new_score);

      if new_score > *best_score {
        continue;
      }

      if matches!(map.at_point(&next_pos), Some(MapElem::None)) {
        let mut path = path.clone();
        path.push(next_pos);
        pq.push(pqe(State2 {
          score: score + cost,
          pos: next_pos,
          dir: new_dir,
          path,
        }));
        visited.insert((new_dir, next_pos));
      }
    }
  }

  // map.print(&|p, e| match e {
  //   _ if best_tiles.contains(&p) => 'O',
  //   _ if p == start => 'S',
  //   _ if p == end => 'E',
  //   MapElem::None => '.',
  //   MapElem::Wall => '#',
  // });
  dbg!(best_tiles.len());
}

fn main() {
  let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
  let input = include_str!("input.txt");
  part2(input);
  part1(input);
}
