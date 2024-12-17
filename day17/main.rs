use itertools::*;

#[derive(Debug)]
struct Computer {
  registers: [u32; 3],
  code: Vec<u32>,
  pc: u32,
}

impl Computer {
  fn combo(&self, operand: u32) -> u32 {
    match operand {
      0..=3 => operand,
      4..=6 => self.registers[(operand - 4) as usize],
      _ => panic!("invalid operand"),
    }
  }
  fn execute_instruction(&mut self, instruction: u32, operand: u32) -> bool {
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
      5 => println!("out: {}", self.combo(operand) & 0b111),
      6 => self.registers[1] = self.registers[0] >> self.combo(operand),
      7 => self.registers[2] = self.registers[0] >> self.combo(operand),
      _ => panic!("invalid instruction"),
    };
    false
  }
  fn get_next(&self) -> (u32, u32) {
    (self.code[self.pc as usize], self.code[(self.pc + 1) as usize])
  }
}

fn parse_register(input: &str) -> u32 {
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

  dbg!(registers);

  Computer { registers, code, pc: 0 }
}

fn main() {
  let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
  let input = include_str!("input.txt");

  let mut computer = parse(input);
  while computer.pc < computer.code.len() as u32 {
    let (instruction, operand) = computer.get_next();
    // dbg!(instruction, operand);
    // dbg!(&computer);
    if !computer.execute_instruction(instruction, operand) {
      computer.pc += 2;
    }
  }
}
