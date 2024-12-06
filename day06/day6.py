input = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""

input = open("input.txt").read()

map = input.splitlines()
width = len(map[0])
height = len(map)

obstructions = set()
guard = None
dir = None
dirs = ["^", ">", "v", "<"]

for y in reversed(range(height)):
  for x in range(width):
    c = map[height-y-1][x]
    if c == ".":
      continue
    elif c == "#":
      obstructions.add((x,y))
    elif c in dirs:
      dir = dirs.index(c)
      guard = (x,y)
    else:
      raise Exception("unknown char")

def simulate(guard, dir, obstructions, width, height):
  dir_coords = [(0,1), (1,0), (0,-1), (-1,0)]
  path = set()
  p = guard
  path.add(p)
  while True:
    p_diff = dir_coords[dir]
    p_next = (p[0]+p_diff[0], p[1]+p_diff[1])
    if not (0 <= p_next[0] < width and 0 <= p_next[1] < height):
      # print(f"p: {p}, dir: {dir}, p_next: {p_next}, p_diff: {p_diff}")
      return len(path)
    if p_next in obstructions:
      dir = (dir + 1) % 4
      continue
    p = p_next
    path.add(p_next)

def get_path_or_check_loops(guard, dir, obstructions, width, height, check_loops = False):
  dir_coords = [(0,1), (1,0), (0,-1), (-1,0)]
  path = []
  p = guard
  path.append((p, dir))
  while True:
    p_diff = dir_coords[dir]
    p_next = (p[0]+p_diff[0], p[1]+p_diff[1])
    if not (0 <= p_next[0] < width and 0 <= p_next[1] < height):
      # print(f"p: {p}, dir: {dir}, p_next: {p_next}, p_diff: {p_diff}")
      return False if check_loops else path
    if p_next in obstructions:
      dir = (dir + 1) % 4
      continue
    if (p_next, dir) in path:
      return True
    p = p_next
    path.append((p_next, dir))
  

# part 1
print(simulate(guard, dir, obstructions, width, height))

# part 2
path = get_path_or_check_loops(guard, dir, obstructions, width, height)
sum = 0

i = 0
points = set(p for p, _ in path[1:])
num_points = len(points)
for p in points:
  print(f"{i}/{num_points}")
  i += 1
  o = obstructions.copy()
  o.add(p)
  loops = get_path_or_check_loops(guard, dir, o, width, height, True)
  sum += 1 if loops else 0
print(sum)

o = obstructions.copy()
o.add((3,3))
