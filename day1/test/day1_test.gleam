import day1
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn part1_test() {
  let test_input =
    "3   4
4   3
2   5
1   3
3   9
3   3"

  day1.part1(test_input)
  |> should.equal(11)
}
