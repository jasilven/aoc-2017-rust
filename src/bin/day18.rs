use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//     snd X plays a sound with a frequency equal to the value of X.
//     set X Y sets register X to the value of Y.
//     add X Y increases register X by the value of Y.
//     mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
//     mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
//     rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
//     jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
//

#[derive(Debug)]
enum Value {
    Int(i64),
    Reg(char),
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        match val.parse::<i64>() {
            Ok(num) => Value::Int(num),
            Err(_) => Value::Reg(val.chars().next().unwrap()),
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

struct Cpu {
    pc: i64,
    regs: HashMap<char, i64>,
    sound: Option<i64>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            regs: HashMap::new(),
            sound: None,
        }
    }

    fn run(&mut self, opcodes: &[OpCode]) -> Result<Option<i64>, String> {
        while self.pc >= 0 && self.pc < opcodes.len() as i64 {
            match &opcodes[self.pc as usize] {
                OpCode::Snd(val) => {
                    let x = match val {
                        Value::Int(i) => i,
                        Value::Reg(ch) => self.regs.get(&ch).unwrap_or(&0),
                    };
                    self.sound = Some(*x);
                    self.pc += 1;
                }
                OpCode::Set(ch, val) => {
                    let y = match val {
                        Value::Int(i) => *i,
                        Value::Reg(ch) => *self.regs.get(&ch).unwrap_or(&0),
                    };
                    self.regs.insert(*ch, y);
                    self.pc += 1;
                }
                OpCode::Add(ch, val) => {
                    let x = *self.regs.get(&ch).unwrap_or(&0);
                    let y = match val {
                        Value::Int(i) => *i,
                        Value::Reg(ch) => *self.regs.get(&ch).unwrap_or(&0),
                    };
                    self.regs.insert(*ch, x + y);
                    self.pc += 1;
                }
                OpCode::Mul(ch, val) => {
                    let x = *self.regs.get(&ch).unwrap_or(&0);
                    let y = match val {
                        Value::Int(i) => *i,
                        Value::Reg(ch) => *self.regs.get(&ch).unwrap_or(&0),
                    };
                    self.regs.insert(*ch, x * y);
                    self.pc += 1;
                }
                OpCode::Mod(ch, val) => {
                    let x = *self.regs.get(&ch).unwrap_or(&0);
                    let y = match val {
                        Value::Int(i) => *i,
                        Value::Reg(ch) => *self.regs.get(&ch).unwrap_or(&0),
                    };
                    self.regs.insert(*ch, x % y);
                    self.pc += 1;
                }
                OpCode::Rcv(val) => {
                    let x = match val {
                        Value::Int(i) => i,
                        Value::Reg(ch) => self.regs.get(&ch).unwrap_or(&0),
                    };
                    if *x != 0i64 {
                        break;
                    }
                    self.pc += 1;
                }
                OpCode::Jgz(val1, val2) => {
                    let x = match val1 {
                        Value::Int(i) => i,
                        Value::Reg(ch) => self.regs.get(&ch).unwrap_or(&0),
                    };
                    if x > &0 {
                        let y = match val2 {
                            Value::Int(i) => i,
                            Value::Reg(ch) => self.regs.get(&ch).unwrap_or(&0),
                        };
                        self.pc += y;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }

        Ok(self.sound)
    }
}

fn parse_ops(fname: &str) -> Result<Vec<OpCode>, Box<dyn std::error::Error>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut splits = line.split(' ');

        match splits.next() {
            Some("snd") => result.push(OpCode::Snd(Value::from(splits.next().unwrap()))),
            Some("set") => result.push(OpCode::Set(
                splits.next().unwrap().chars().next().unwrap(),
                Value::from(splits.next().unwrap()),
            )),
            Some("add") => result.push(OpCode::Add(
                splits.next().unwrap().chars().next().unwrap(),
                Value::from(splits.next().unwrap()),
            )),
            Some("mul") => result.push(OpCode::Mul(
                splits.next().unwrap().chars().next().unwrap(),
                Value::from(splits.next().unwrap()),
            )),
            Some("mod") => result.push(OpCode::Mod(
                splits.next().unwrap().chars().next().unwrap(),
                Value::from(splits.next().unwrap()),
            )),
            Some("rcv") => result.push(OpCode::Rcv(Value::from(splits.next().unwrap()))),
            Some("jgz") => result.push(OpCode::Jgz(
                Value::from(splits.next().unwrap()),
                Value::from(splits.next().unwrap()),
            )),
            None | Some(_) => return Err("Unknown opcode".into()),
        }
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops = parse_ops("resources/day18_input.txt")?;
    let mut cpu = Cpu::new();
    cpu.run(&ops)?;

    println!("part 1: {:?}", &cpu.sound.unwrap());

    Ok(())
}
