input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"

input = File.read!("input.txt")

parse_line = fn line ->
  [left, right] = String.split(line, ": ")
  result = String.to_integer(left)
  operands = right |> String.split() |> Enum.map(&String.to_integer/1)
  {result, operands}
end

is_correct = fn line ->
  {expected_result, [head | tail]} = line

  Enum.reduce(tail, [head], fn num, acc ->
    acc |> Enum.flat_map(fn x -> [num + x, num * x] end)
  end)
  |> Enum.member?(expected_result)
end

concat = fn a, b ->
  String.to_integer(Integer.to_string(a) <> Integer.to_string(b))
end

is_correct2 = fn line ->
  {expected_result, [head | tail]} = line

  Enum.reduce(tail, [head], fn num, acc ->
    acc |> Enum.flat_map(fn x -> [num + x, num * x, concat.(x, num)] end)
  end)
  |> Enum.member?(expected_result)
end

lines = String.split(input, "\n")

part1 =
  lines
  |> Enum.map(parse_line)
  |> Enum.filter(is_correct)
  |> Enum.map(fn {expect_result, _} -> expect_result end)
  |> Enum.sum()

part2 =
  lines
  |> Enum.map(parse_line)
  |> Enum.filter(is_correct2)
  |> Enum.map(fn {expect_result, _} -> expect_result end)
  |> Enum.sum()

IO.inspect(part1)
IO.inspect(part2)
