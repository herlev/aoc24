import gleam/int
import gleam/io
import gleam/list
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

fn u(result: Result(value, error)) -> value {
  case result {
    Ok(v) -> v
    Error(_) -> panic
  }
}

pub fn main() {
  let input =
    simplifile.read("input.txt")
    |> u
    |> string.trim
  io.debug(part1(input))
}
