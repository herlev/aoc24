# https://nim-by-example.github.io/hello_world/
import strutils
import std/sequtils
import std/sugar
import std/math

iterator window[T](s: seq[T], size: int): seq[T] =
  if size > 0 and size <= s.len:
    for i in 0 ..< s.len - size + 1:
      yield s[i ..< i + size]

# let s = """7 6 4 2 1
# 1 2 7 8 9
# 9 7 6 2 1
# 1 3 2 4 5
# 8 6 4 4 1
# 1 3 6 7 9"""

let s = readFile("input.txt")

proc is_safe1(s: seq[int]): bool =
  let diffs = s.window(2).toSeq.mapIt(it[0] - it[1])
  let increasing_or_decreasing = diffs.mapIt(it.sgn()).toSeq
    .window(2).toSeq
    .mapIt(it[0] != 0 and it[0] == it[1]).toSeq
    .all(x => x)
  let is_at_most_increasing_by_3 = diffs.mapIt(it.abs() <= 3).all(x => x)
  return increasing_or_decreasing and is_at_most_increasing_by_3

proc is_safe2(s: seq[int]): bool =
  let diffs = s.window(2).toSeq.mapIt(it[0] - it[1])
  let increasing_or_decreasing = diffs.mapIt(it.sgn()).toSeq
    .window(2).toSeq
    .mapIt(it[0] != 0 and it[0] == it[1]).toSeq
    .all(x => x)
  let is_at_most_increasing_by_3 = diffs.mapIt(it.abs() <= 3).all(x => x)

  if increasing_or_decreasing and is_at_most_increasing_by_3:
    return true
  for i in 0..s.len()-1:
    var copy = s
    copy.delete(i)
    if is_safe1(copy):
      return true

  return false

let part1 = s.split('\n').mapIt(it.split(' ').map(parseInt)).map(is_safe1).filterIt(it).len()
echo part1
let part2 = s.split('\n').mapIt(it.split(' ').map(parseInt)).map(is_safe2).filterIt(it).len()
echo part2
