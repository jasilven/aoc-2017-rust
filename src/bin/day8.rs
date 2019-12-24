use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
struct MyError(String);

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError({})", self.0)
    }
}

impl From<&str> for MyError {
    fn from(s: &str) -> Self {
        MyError(s.to_owned())
    }
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError(error.to_string())
    }
}

type Op = (String, String, isize, String, String, isize);

fn parse_line(line: &str) -> Op {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (==|!=|<|>|<=|>=) (-?\d+)$")
                .expect("unable to create regex");
    }
    let caps = RE.captures(&line).unwrap();
    (
        caps[1].to_string(),
        caps[2].to_string(),
        caps[3].parse::<isize>().expect("unable to parse 3th value"),
        caps[4].to_string(),
        caps[5].to_string(),
        caps[6].parse::<isize>().expect("unable to parse 5th value"),
    )
}

fn eval_exp(a: isize, oper: &str, b: isize) -> Result<bool> {
    let result = match oper {
        "==" => a == b,
        "!=" => a != b,
        "<" => a < b,
        ">" => a > b,
        "<=" => a <= b,
        ">=" => a >= b,
        _ => return Err("unknown operator".into()),
    };
    Ok(result)
}

fn execute(ops: &[Op]) -> Result<(HashMap<String, isize>, Option<isize>)> {
    let mut regs = HashMap::<String, isize>::new();
    let mut all_time_max: Option<isize> = None;
    for op in ops {
        let a = match regs.get(&op.3) {
            Some(val) => *val,
            None => 0,
        };

        if eval_exp(a, &op.4, op.5)? {
            if !regs.contains_key(&op.0) {
                regs.insert(op.0.to_string(), 0);
            }
            let reg = regs.get_mut(&op.0).unwrap();
            let reg_val = match op.1.as_str() {
                "inc" => *reg + op.2,
                "dec" => *reg - op.2,
                _ => return Err("only inc or dec ops supported".into()),
            };
            *reg = reg_val;

            if all_time_max == None {
                all_time_max = Some(reg_val);
            } else {
                let max = std::cmp::max(all_time_max.unwrap(), reg_val);
                all_time_max = Some(max);
            }
        }
    }
    Ok((regs, all_time_max))
}

fn solve(ops: &[Op]) -> (isize, isize) {
    let (regs, all_time_max) = execute(ops).expect("execute error");
    let part1 = *regs.values().max().expect("cannot find max");
    (part1, all_time_max.expect("all time max not found"))
}

fn parse_input(fname: &str) -> Result<Vec<Op>> {
    let mut result = vec![];
    let file = File::open(fname).or_else(Err)?;
    for line in BufReader::new(&file).lines() {
        let line = line?;
        result.push(parse_line(&line));
    }
    Ok(result)
}

fn main() {
    let input = parse_input("resources/day8_input.txt").expect("unable to parse input file");
    let (part1, part2) = solve(&input);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

mod tests {

    #[test]
    fn solve_test() {
        use super::*;
        let input = parse_input("resources/day8_testdata.txt").unwrap();
        let (part1, part2) = solve(&input);
        assert_eq!(1, part1);
        assert_eq!(10, part2);
    }
}
