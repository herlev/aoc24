// https://typst.app/docs/reference/scripting/

#{
let p(x) = repr(x) + "\n"
let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
let input = read("input.txt")
let input = input.split("\n\n")
let rules = input.at(0).split("\n").map(line => line.split("|").map(int))
let updates = input.at(1).split("\n").map(line => line.split(",").map(int))

let is_correct_order(update) = {
  let previous = ()
  for num in update {
    let must_not_be_before = rules.filter(((left, right)) => left == num).map(((left, right)) => right)
    for prev in previous {
      if prev in must_not_be_before {
        return false
      }
    }
    previous.push(num)
  }
  true
}

let fix_order(update) = {
  for i in range(update.len()-1) {
    for j in range(i+1, update.len()) {
      if (update.at(j), update.at(i)) in rules {
        let tmp = update.at(i)
        update.at(i) = update.at(j)
        update.at(j) = tmp
      }
    }
  }
  update
}

let middle_num(update) = {
  update.at(int(update.len()/2))
}

let part1 = updates.filter(is_correct_order).map(middle_num).sum()
let part2 = updates.filter(update => not is_correct_order(update)).map(fix_order).map(middle_num).sum()
"Part1: " + repr(part1) + "\n"
"Part2: " + repr(part2) + "\n"
}
