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

fn part1(input: &str) {
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

fn part2(input: &str) {
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
  let map = Grid2::from_char_grid(map, &mut |p, c| match c {
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
  pos.x = 2 * pos.x;
  let mut new_map: Grid2<MapElem> = Grid2::new(0..=2 * (map.x_range.end() + 1) - 1, map.y_range.clone());

  for p in map.iter() {
    let new_p = Point2::new(2 * p.x, p.y);
    let e = map.at_point(&p).unwrap();
    if matches!(e, MapElem::Wall) {
      *new_map.at_point_mut(&new_p).unwrap() = MapElem::Wall;
      *new_map.at_point_mut(&(new_p + Direction::Right.to_point())).unwrap() = MapElem::Wall;
    }
    if matches!(e, MapElem::Box) {
      *new_map.at_point_mut(&new_p).unwrap() = MapElem::Box;
    }
  }

  // print_map2(&new_map, pos);
  for m in moves {
    (new_map, pos) = perform_move2(new_map, pos, m);
  }
  let part2: i32 = new_map
    .iter()
    .filter_map(|p| match new_map.at_point(&p) {
      Some(MapElem::Box) => Some(p),
      _ => None,
    })
    .map(|p| get_gps_coordinate(&map, p))
    .sum();
  dbg!(part2);
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
  // <^^>>>vv<v>>v<<";
  //   let input = "#######
  // #...#.#
  // #.....#
  // #..OO@#
  // #..O..#
  // #.....#
  // #######
  // <vv<<^^<<^^";
  let input = include_str!("input.txt");
  part1(input);
  part2(input);
}

fn has_box(map: &Map, pos: Pos) -> bool {
  matches!(map.at_point(&pos), Some(MapElem::Box))
    || matches!(map.at_point(&(pos + Direction::Left.to_point())), Some(MapElem::Box))
}

fn boxes_right(map: &Map, pos: Pos) -> (Vec<Point2<i32>>, bool) {
  let new_pos = pos + Direction::Right.to_point();
  match map.at_point(&new_pos).unwrap() {
    MapElem::None => (vec![], true),
    MapElem::Box => {
      let next_pos = new_pos + Direction::Right.to_point();
      assert!(matches!(map.at_point(&next_pos), Some(MapElem::None)));
      let (boxes, can_move) = boxes_right(map, next_pos);
      ([vec![new_pos], boxes].concat(), can_move)
    }
    MapElem::Wall => (vec![], false),
  }
}

fn boxes_left(map: &Map, pos: Pos) -> (Vec<Point2<i32>>, bool) {
  let new_pos = pos + Direction::Left.to_point();
  match map.at_point(&new_pos).unwrap() {
    MapElem::Box => {
      let (boxes, can_move) = boxes_left(map, new_pos);
      ([vec![new_pos], boxes].concat(), can_move)
    }
    MapElem::None => {
      if has_box(map, new_pos) {
        boxes_left(map, new_pos)
      } else {
        (vec![], true)
      }
    }
    MapElem::Wall => (vec![], false),
  }
}

fn boxes_up_down(map: &Map, pos: Pos, dir: Direction) -> (Vec<Point2<i32>>, bool) {
  assert!(matches!(dir, Direction::Up | Direction::Down));
  let new_pos = pos + dir.to_point();
  match map.at_point(&new_pos).unwrap() {
    MapElem::Wall => (vec![], false),
    MapElem::None => {
      if has_box(map, new_pos) {
        // new_pos is right side of box
        let (boxes_l, can_move_l) = boxes_up_down(map, new_pos + Direction::Left.to_point(), dir);
        let (boxes_r, can_move_r) = boxes_up_down(map, new_pos, dir);
        (
          [vec![new_pos + Direction::Left.to_point()], boxes_l, boxes_r].concat(),
          can_move_l && can_move_r,
        )
      } else {
        (vec![], true)
      }
    }
    MapElem::Box => {
      // new_pos is left side of box
      let (boxes_l, can_move_l) = boxes_up_down(map, new_pos, dir);
      let (boxes_r, can_move_r) = boxes_up_down(map, new_pos + Direction::Right.to_point(), dir);
      ([vec![new_pos], boxes_l, boxes_r].concat(), can_move_l && can_move_r)
    }
  }
}

fn boxes(map: &Map, pos: Pos, dir: Direction) -> (Vec<Point2<i32>>, bool) {
  match dir {
    Direction::Up | Direction::Down => boxes_up_down(map, pos, dir),
    Direction::Right => boxes_right(map, pos),
    Direction::Left => boxes_left(map, pos),
  }
}

fn perform_move2(mut map: Map, pos: Pos, dir: Direction) -> (Map, Pos) {
  let (boxes, can_move) = boxes(&map, pos, dir);
  if !can_move {
    return (map, pos);
  }
  for b in &boxes {
    *map.at_point_mut(&b).unwrap() = MapElem::None;
  }
  for b in boxes {
    *map.at_point_mut(&(b + dir.to_point())).unwrap() = MapElem::Box;
  }
  (map, pos + dir.to_point())
}

fn print_map2(map: &Map, pos: Pos) {
  let mut i = iproduct!(map.y_range.clone().rev(), map.x_range.clone()).map(|(y, x)| Point2::from((x, y)));
  let mut prev_y = None;
  while let Some(p) = i.next() {
    if prev_y == Some(p.y + 1) {
      print!("\n");
    }
    let s = match map.at_point(&p).unwrap() {
      _ if p == pos => "@",
      MapElem::None => ".",
      MapElem::Box => {
        assert!(matches!(map.at_point(&i.next().unwrap()).unwrap(), MapElem::None));
        "[]"
      }
      MapElem::Wall => "#",
    };
    print!("{s}");
    prev_y = Some(p.y);
  }
  print!("\n");
}
