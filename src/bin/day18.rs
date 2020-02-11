use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    result: Option<i64>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            regs: HashMap::new(),
            result: None,
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
                    self.result = Some(*x);
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

        Ok(self.result)
    }
}

struct Cpu2 {
    pc: i64,
    regs: HashMap<char, i64>,
    result: i64,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
}

impl Cpu2 {
    fn new(sender: Sender<i64>, receiver: Receiver<i64>, p_val: i64) -> Cpu2 {
        let mut regs = HashMap::new();
        regs.insert('p', p_val);
        Cpu2 {
            pc: 0,
            regs,
            result: 0,
            sender,
            receiver,
        }
    }

    fn run(&mut self, opcodes: &[OpCode]) -> Result<i64, String> {
        while self.pc >= 0 && self.pc < opcodes.len() as i64 {
            match &opcodes[self.pc as usize] {
                OpCode::Snd(val) => {
                    let x = match val {
                        Value::Int(i) => i,
                        Value::Reg(ch) => self.regs.get(&ch).unwrap_or(&0),
                    };
                    self.sender.send(*x).unwrap();
                    self.result += 1;
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
                OpCode::Rcv(Value::Reg(ch)) => {
                    let d = Duration::from_millis(500);
                    match self.receiver.recv_timeout(d).map_err(|e| e.to_string()) {
                        Ok(i) => {
                            self.regs.insert(*ch, i);
                            self.pc += 1;
                        }
                        _ => break,
                    }
                }
                OpCode::Rcv(Value::Int(_)) => {
                    return Err("Wrong opcode: OpCode::Rcv(Value::Int(x))".into())
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

        Ok(self.result)
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

    println!("part 1: {:?}", &cpu.result.unwrap());

    let (sa, ra) = mpsc::channel();
    let (sb, rb) = mpsc::channel();

    let mut cpu2_a = Cpu2::new(sa, rb, 0);
    let mut cpu2_b = Cpu2::new(sb, ra, 1);
    let ops2 = ops.clone();

    let t1 = thread::spawn(move || {
        cpu2_a.run(&ops).unwrap();
        cpu2_a.result
    });
    let t2 = thread::spawn(move || {
        cpu2_b.run(&ops2).unwrap();
        cpu2_b.result
    });

    let _ = t1.join().unwrap();
    let res2 = t2.join().unwrap();

    println!("part 2: {}", &res2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let ops = parse_ops("resources/day18_testdata.txt").unwrap();
        let mut cpu = Cpu::new();
        cpu.run(&ops).unwrap();

        assert_eq!(4, cpu.result.unwrap());
    }
}
