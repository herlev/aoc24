use itertools::{iproduct, Itertools};

fn parse_block(block: Vec<Vec<char>>, is_key: bool) -> [i32; 5] {
  let mut r = [0; 5];
  for col in 0..5 {
    for i in 0..=5 {
      let row = if is_key { 5 - i } else { 1 + i };
      if block[row][col] == '.' {
        r[col] = i as i32;
        break;
      }
      let s = block.iter().map(|row| row.iter().join("")).join("\n");
      assert!(i != 5, "{s}, {col:?}, {is_key}");
    }
  }
  r
}

fn main() {
  let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
  let input = include_str!("input.txt");
  let (locks, keys): (Vec<_>, Vec<_>) = input
    .trim()
    .split("\n\n")
    .map(|block| block.lines().map(|line| line.chars().collect_vec()).collect_vec())
    .partition(|block| match block[0][0] {
      '#' => true,  // lock
      '.' => false, // key
      _ => todo!(),
    });
  let locks = locks.into_iter().map(|b| parse_block(b, false)).collect_vec();
  let keys = keys.into_iter().map(|b| parse_block(b, true)).collect_vec();
  let combos = iproduct!(locks, keys);
  let part1 = combos
    .filter(|(lock, key)| lock.into_iter().zip(key.into_iter()).all(|(lp, kp)| 5 - lp - kp >= 0))
    .count();
  dbg!(part1);
}
