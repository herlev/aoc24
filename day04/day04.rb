input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
input = File.read('input.txt')

lines = input.split("\n")
$grid = lines.map(&:chars)

$directions = [
  [0, 1],   # Right
  [0, -1],  # Left
  [1, 0],   # Down
  [-1, 0],  # Up
  [1, 1],   # Down-Right
  [1, -1],  # Down-Left
  [-1, 1],  # Up-Right
  [-1, -1]  # Up-Left
]

def assert(condition, message)
  raise "Assertion failed: #{message}" unless condition
end

def check_xmas(grid, x, y, directions)
  target = "XMAS"
  sum = 0
  directions.each do |dx, dy|
    matched = true
    target.each_char.with_index do |char, i|
      nx, ny = x + dx * i, y + dy * i
      if nx < 0 || nx >= grid.length || ny < 0 || ny >= grid[0].length || grid[ny][nx] != char
        matched = false
        break
      end
    end
    if matched
      sum += 1
    end
  end
  sum
end

def part1
  sum = 0
  $grid.each_with_index do |row, y|
    row.each_with_index do |char, x|
      if char == 'X'
        sum += check_xmas($grid, x, y, $directions)
      end
    end
  end
  pp sum
end

def check_mas(grid, x, y, directions)
  diag1 = (grid[y-1][x-1] == "M" && grid[y+1][x+1] == "S") || (grid[y-1][x-1] == "S" && grid[y+1][x+1] == "M")
  diag2 = (grid[y-1][x+1] == "M" && grid[y+1][x-1] == "S") || (grid[y-1][x+1] == "S" && grid[y+1][x-1] == "M")
  diag1 and diag2
end

def part2
  width = $grid[0].length
  height = $grid.length
  sum = 0
  $grid.each_with_index.filter{|_, y| 0 < y && y < height-1}.each do |row, y|
    row.each_with_index.filter{|_, x| 0 < x && x < width-1}.each do |char, x|
      if char == 'A'
        sum += 1 if check_mas($grid, x, y, $directions) else 0
      end
    end
  end
  pp sum
end

# pp $grid
check_mas($grid, 2, 1, $directions)

assert(part1() == 2613, "part1 failed")
assert(part2() == 1905, "part2 failed")
