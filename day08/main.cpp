#include <format>
#include <fstream>
#include <print>
#include <ranges>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std::views;
using namespace std::ranges;

struct Point {
  Point(int x, int y) noexcept : x(x), y(y) {}
  int x, y;
  auto operator==(const Point &other) const noexcept -> bool {
    return x == other.x && y == other.y;
  }

  auto is_within_grid(int width, int height) -> bool {
    return 0 <= x && x < width && 0 <= y && y < height;
  }

  auto operator-(const Point &other) const noexcept -> Point {
    return Point(x - other.x, y - other.y);
  }
  auto operator+(const Point &other) const noexcept -> Point {
    return Point(x + other.x, y + other.y);
  }
  auto operator*(const Point &other) const noexcept -> Point {
    return Point(x * other.x, y * other.y);
  }

  static auto hash(Point const &p) noexcept -> uint64_t {
    return ((uint64_t)p.x) << 32 | (uint64_t)p.y;
  }
  static auto to_string(Point const &p) noexcept -> std::string {
    return std::format("({},{})", p.x, p.y);
  }
};

namespace std {
template <> struct hash<Point> {
  auto operator()(const Point &p) const noexcept -> std::size_t {
    return Point::hash(p);
  }
};
} // namespace std

auto main() -> int {
  //   std::string input = R"(............
  // ........0...
  // .....0......
  // .......0....
  // ....0.......
  // ......A.....
  // ............
  // ............
  // ........A...
  // .........A..
  // ............
  // ............ )";
  std::ifstream ifs("input.txt");
  std::string input((std::istreambuf_iterator<char>(ifs)),
                    (std::istreambuf_iterator<char>()));
  auto lines_vec =
      input | views::split('\n') |
      views::transform([](auto r) { return std::string(r.data(), r.size()); }) |
      std::ranges::to<std::vector>();

  int width = lines_vec[0].size();
  int height = lines_vec.size();

  std::unordered_map<char, std::vector<Point>> antennas;

  for (int y : iota(0, height) | reverse) {
    for (int x : iota(0, width)) {
      char c = lines_vec[height - y - 1][x];
      auto p = Point(x, y);
      if (c == '.') {
        continue;
      }
      if (!antennas.contains(c)) {
        antennas.emplace(c, std::vector{p});
      } else {
        antennas.at(c).emplace_back(p);
      }
    }
  }

  std::unordered_set<Point> antinodes_part1;
  std::unordered_set<Point> antinodes_part2;
  for (const auto &[antenna, points] : antennas) {
    if (points.size() <= 1) {
      continue;
    }

    for (int i = 0; i < points.size(); i++) {
      for (int j = i + 1; j < points.size(); j++) {
        Point a = points[i];
        Point b = points[j];
        Point diff = a - b;
        Point p1 = a + diff;
        Point p2 = b - diff;

        if (p1.is_within_grid(width, height)) {
          antinodes_part1.insert(p1);
        }
        if (p2.is_within_grid(width, height)) {
          antinodes_part1.insert(p2);
        }

        antinodes_part2.insert(a);
        antinodes_part2.insert(b);
        while (p1.is_within_grid(width, height)) {
          antinodes_part2.insert(p1);
          p1 = p1 + diff;
        }
        while (p2.is_within_grid(width, height)) {
          antinodes_part2.insert(p2);
          p2 = p2 - diff;
        }
        // std::println("A: {}, B: {}, P1: {}, P2: {}", Point::to_string(a),
        //              Point::to_string(b), Point::to_string(p1),
        //              Point::to_string(p2));
      }
    }

    std::string points_s = points | views::transform(Point::to_string) |
                           views::join_with(',') | to<std::string>();
    // std::println("{}: [{}]", antenna, points_s);
  }

  uint sum1 = 0;
  uint sum2 = 0;
  for (const auto &antinode : antinodes_part1) {
    sum1 += 1;
  }
  for (const auto &antinode : antinodes_part2) {
    lines_vec[height - antinode.y - 1][antinode.x] = '#';
    sum2 += 1;
  }
  // std::println("{}", lines_vec | views::join_with('\n') | to<std::string>());
  std::println("part1: {}", sum1);
  std::println("part2: {}", sum2);
}
