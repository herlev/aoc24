fn parse_num(s: &str) -> Option<(u32, &str)> {
  match s
    .chars()
    .take_while(char::is_ascii_digit)
    .fold((0, 0), |(count, acc), d| (count + 1, 10 * acc + d as u32 - b'0' as u32))
  {
    (0, _) => None,
    (count, num) => Some((num, &s[count..])),
  }
}

fn parse_mul(s: &str) -> Option<(u32, &str)> {
  let s = s.strip_prefix("mul(")?;
  let (a, s) = parse_num(s)?;
  let s = s.strip_prefix(",")?;
  let (b, s) = parse_num(s)?;
  let s = s.strip_prefix(")")?;
  Some((a * b, s))
}

fn part_1_and_2(mut input: &str) {
  let mut mul_enabled = true;
  let mut part1 = 0;
  let mut part2 = 0;

  while !input.is_empty() {
    if let Some(rest) = input.strip_prefix("do()") {
      mul_enabled = true;
      input = rest;
      continue;
    }
    if let Some(rest) = input.strip_prefix("don't()") {
      mul_enabled = false;
      input = rest;
      continue;
    }
    if let Some((product, rest)) = parse_mul(input) {
      if mul_enabled {
        part2 += product;
      }
      part1 += product;
      input = rest;
      continue;
    }
    input = &input[1..];
  }
  dbg!(part1);
  dbg!(part2);
}

fn main() {
  // let input = include_str!("input.txt");
  let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
  part_1_and_2(input);
}
