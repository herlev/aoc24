use itertools::*;
use rust_lib::{Direction, Grid2, Point2};

#[derive(Clone, Debug, Default)]
enum MapElem {
  #[default]
  None,
  Box,
  Wall,
}

type Map = Grid2<MapElem>;
type Pos = Point2<i32>;

fn print_map(map: &Map, pos: Pos) {
  map.print(&|p, e| match e {
    _ if p == pos => '@',
    MapElem::None => '.',
    MapElem::Box => 'O',
    MapElem::Wall => '#',
  });
}

fn perform_move(mut map: Map, pos: Pos, m: Direction) -> (Map, Pos) {
  let mut tmp_pos = pos;
  let o = loop {
    tmp_pos += m.to_point();
    match map.at_point(&tmp_pos) {
      Some(MapElem::None) => break Some((pos + m.to_point(), tmp_pos)), // ??
      Some(MapElem::Wall) => break None,
      Some(MapElem::Box) => (),
      None => todo!(),
    }
  };
  let Some((new_pos, end_pos)) = o else { return (map, pos) };
  *map.at_point_mut(&end_pos).unwrap() = MapElem::Box;
  *map.at_point_mut(&new_pos).unwrap() = MapElem::None;
  (map, new_pos)
}

fn get_gps_coordinate(map: &Map, pos: Pos) -> i32 {
  100 * (map.y_range.end() - pos.y) + pos.x
}

fn main() {
  let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
  //   let input = "########
  // #..O.O.#
  // ##@.O..#
  // #...O..#
  // #.#.O..#
  // #...O..#
  // #......#
  // ########
  let input = include_str!("input.txt");

  // <^^>>>vv<v>>v<<";

  let (map, moves) = input.split_once("\n\n").unwrap();
  let moves = moves
    .lines()
    .join("")
    .chars()
    .map(|c| match c {
      '^' => Direction::Up,
      'v' => Direction::Down,
      '<' => Direction::Left,
      '>' => Direction::Right,
      _ => todo!(),
    })
    .collect_vec();
  let mut pos = None;
  let mut map = Grid2::from_char_grid(map, &mut |p, c| match c {
    '.' => MapElem::None,
    'O' => MapElem::Box,
    '#' => MapElem::Wall,
    '@' => {
      pos = Some(p);
      MapElem::None
    }
    _ => todo!("{c}"),
  });
  let mut pos = pos.unwrap();
  for m in moves {
    (map, pos) = perform_move(map, pos, m);
    // dbg!(&m);
    // print_map(&map, pos);
  }

  let part1: i32 = map
    .iter()
    .filter_map(|p| match map.at_point(&p) {
      Some(MapElem::Box) => Some(p),
      _ => None,
    })
    .map(|p| get_gps_coordinate(&map, p))
    .sum();
  dbg!(part1);
}
