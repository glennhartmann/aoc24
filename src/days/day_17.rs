use std::io::Write;

use aoclib_rs::{prep_io, printwriteln};

#[derive(Copy, Clone, Debug)]
enum Operand {
    Literal(u8),
    RegA,
    RegB,
    RegC,
    Invalid,
}

impl Operand {
    fn new(op: u8) -> Operand {
        match op {
            0..=3 => Operand::Literal(op),
            4 => Operand::RegA,
            5 => Operand::RegB,
            6 => Operand::RegC,
            7 => Operand::Invalid,
            _ => panic!("invalid operand: {}", op),
        }
    }

    fn literal_value(self) -> u8 {
        match self {
            Operand::Literal(u) => u,
            Operand::RegA => 4,
            Operand::RegB => 5,
            Operand::RegC => 6,
            Operand::Invalid => 7,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn new(op: u8) -> Opcode {
        match op {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("invalid opcode: {}", op),
        }
    }
}

struct Cpu {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
    prog: Vec<(Opcode, Operand)>,
    output: String,
}

impl Cpu {
    fn new(a: u64, b: u64, c: u64, prog: Vec<u8>) -> Cpu {
        let mut p = Vec::with_capacity(prog.len() / 2);
        let mut i = 0;
        while i < prog.len() {
            p.push((Opcode::new(prog[i]), Operand::new(prog[i + 1])));
            i += 2;
        }

        println!("prog: {:?}", p);

        Cpu {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            ip: 0,
            prog: p,
            output: "".to_owned(),
        }
    }

    fn run(&mut self) -> String {
        loop {
            let (output, halt) = self.execute_instr();
            if let Some(out) = output {
                if !self.output.is_empty() {
                    self.output.push(',');
                }
                self.output.push_str(&out);
            }

            if halt {
                break;
            }
        }

        self.output.clone()
    }

    fn execute_instr(&mut self) -> (Option<String>, bool) {
        let op = self.prog[self.ip];
        let combo_operand_value = match op.1 {
            Operand::Literal(u) => u as u64,
            Operand::RegA => self.reg_a,
            Operand::RegB => self.reg_b,
            Operand::RegC => self.reg_c,
            Operand::Invalid => u64::MAX,
        };
        let literal_operand_value = op.1.literal_value();

        let mut output = None;

        match op.0 {
            Opcode::Adv => {
                self.reg_a /= 1 << combo_operand_value;
                self.ip += 1;
            }
            Opcode::Bxl => {
                self.reg_b ^= literal_operand_value as u64;
                self.ip += 1;
            }
            Opcode::Bst => {
                self.reg_b = combo_operand_value % 8;
                self.ip += 1;
            }
            Opcode::Jnz => {
                if self.reg_a == 0 {
                    self.ip += 1;
                } else {
                    self.ip = (literal_operand_value / 2) as usize;
                }
            }
            Opcode::Bxc => {
                self.reg_b ^= self.reg_c;
                self.ip += 1;
            }
            Opcode::Out => {
                output = Some((combo_operand_value % 8).to_string());
                self.ip += 1;
            }
            Opcode::Bdv => {
                self.reg_b = self.reg_a / (1 << combo_operand_value);
                self.ip += 1;
            }
            Opcode::Cdv => {
                self.reg_c = self.reg_a / (1 << combo_operand_value);
                self.ip += 1;
            }
        };

        (output, self.ip >= self.prog.len())
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 17).unwrap();

    let reg_a = contents[0]
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let reg_b = contents[1]
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let reg_c = contents[2]
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    let prog = contents[4]
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cpu = Cpu::new(reg_a, reg_b, reg_c, prog);
    let output = cpu.run();

    printwriteln!(writer, "part 1: {}", output).unwrap();
}
