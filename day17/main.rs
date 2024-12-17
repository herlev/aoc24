use itertools::*;

#[derive(Debug)]
struct Computer {
  registers: [u64; 3],
  code: Vec<u64>,
  pc: u64,
}

impl Computer {
  fn combo(&self, operand: u64) -> u64 {
    match operand {
      0..=3 => operand,
      4..=6 => self.registers[(operand - 4) as usize],
      _ => panic!("invalid operand"),
    }
  }
  fn execute_instruction(&mut self, instruction: u64, operand: u64) -> bool {
    match instruction {
      0 => self.registers[0] >>= self.combo(operand),
      1 => self.registers[1] ^= operand,
      2 => self.registers[1] = self.combo(operand) & 0b111,
      3 => {
        if self.registers[0] != 0 {
          self.pc = operand;
          return true;
        }
      }
      4 => self.registers[1] ^= self.registers[2],
      5 => print!("{},", self.combo(operand) & 0b111),
      6 => self.registers[1] = self.registers[0] >> self.combo(operand),
      7 => self.registers[2] = self.registers[0] >> self.combo(operand),
      _ => panic!("invalid instruction"),
    };
    false
  }
  fn get_next(&self) -> (u64, u64) {
    (self.code[self.pc as usize], self.code[(self.pc + 1) as usize])
  }
}

fn parse_register(input: &str) -> u64 {
  input.split_once(": ").unwrap().1.parse().unwrap()
}

fn parse(input: &str) -> Computer {
  let (registers, program) = input.split_once("\n\n").unwrap();
  let registers = registers.lines().map(parse_register).collect_vec().try_into().unwrap();
  let code = program
    .split_once(": ")
    .unwrap()
    .1
    .split(",")
    .map(|s| s.parse().unwrap())
    .collect();

  Computer { registers, code, pc: 0 }
}

fn part1(input: &str) {
  let mut computer = parse(input);
  while computer.pc < computer.code.len() as u64 {
    let (instruction, operand) = computer.get_next();
    if !computer.execute_instruction(instruction, operand) {
      computer.pc += 2;
    }
  }
  println!();
}

fn compute(a: u64) -> u64 {
  // Got this expression by writing out the instructions and substituting
  ((((a & 0b111) ^ 7) ^ (a >> ((a & 0b111) ^ 7))) ^ 4) & 7
}

fn find(a: u64, goal: u64) -> Vec<u64> {
  (0..=7).map(|i| (a << 3) | i).filter(|&a| compute(a) == goal).collect()
}

fn dfs(v: &[u64], a: u64) -> Option<u64> {
  if v.is_empty() {
    return Some(a);
  }
  find(a, v[0]).into_iter().find_map(|new_a| dfs(&v[1..], new_a))
}

fn part2() {
  let goal = [2, 4, 1, 7, 7, 5, 4, 1, 1, 4, 5, 5, 0, 3, 3, 0];
  dbg!(dfs(&goal.into_iter().rev().collect_vec(), 0));
}

fn main() {
  let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
  let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
  let input = include_str!("input.txt");
  part1(input);
  part2();
}
