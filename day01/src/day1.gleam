import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/set
import gleam/string
import simplifile

fn not(v: value, f: fn(value) -> Bool) -> Bool {
  !f(v)
}

fn unwrap(
  v: value,
  f: fn(value) -> Result(othervalue, someothervalue),
) -> othervalue {
  case f(v) {
    Ok(v) -> v
    _ -> panic
  }
}

pub fn part1(input: String) -> Int {
  let lines =
    input
    |> string.split("\n")

  let pairs =
    lines
    |> list.map(string.split(_, " "))
    |> list.map(list.filter(_, not(_, string.is_empty)))
    |> list.map(list.map(_, unwrap(_, int.parse)))
    |> list.map(fn(l) {
      let assert [left, right] = l
      #(left, right)
    })
  let #(left, right) =
    pairs
    |> list.unzip

  let sorted_zip =
    list.zip(
      left
        |> list.sort(int.compare),
      right
        |> list.sort(int.compare),
    )

  let differences =
    sorted_zip
    |> list.map(fn(t) {
      let #(a, b) = t
      { a - b }
      |> int.absolute_value
    })
  int.sum(differences)
}

fn count_to_dict(l: List(Int)) -> dict.Dict(Int, Int) {
  list.fold(l, dict.new(), fn(d, n) {
    n
    |> dict.get(d, _)
    |> result.unwrap(0)
    |> int.add(1)
    |> dict.insert(d, n, _)
  })
}

pub fn inspect(v: value) -> value {
  io.debug(v)
  v
}

fn list_to_tuple2(l: List(value)) -> #(value, value) {
  let assert [left, right] = l
  #(left, right)
}

pub fn part2(input: String) -> Int {
  let lines =
    input
    |> string.split("\n")

  let #(left, right) =
    lines
    |> list.map(string.split(_, " "))
    |> list.map(list.filter(_, not(_, string.is_empty)))
    |> list.map(list.map(_, unwrap(_, int.parse)))
    |> list.map(list_to_tuple2)
    |> list.unzip

  let ld = count_to_dict(left)
  let rd = count_to_dict(right)

  let common_numbers =
    set.intersection(
      ld
        |> dict.keys
        |> set.from_list,
      rd
        |> dict.keys
        |> set.from_list,
    )

  let r =
    common_numbers
    |> set.to_list
    |> list.map(fn(n) { n * u(dict.get(ld, n)) * u(dict.get(rd, n)) })
    |> int.sum
  r
}

fn u(result: Result(value, error)) -> value {
  case result {
    Ok(v) -> v
    Error(_) -> panic
  }
}

pub fn main() {
  let _test_input =
    "3   4
  4   3
  2   5
  1   3
  3   9
  3   3"
  let input =
    simplifile.read("input.txt")
    |> u
    |> string.trim
  io.debug(part1(input))
  io.debug(part2(input))
}
