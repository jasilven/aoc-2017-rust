use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

type Op = (String, String, isize, String, String, isize);

fn parse_line(line: String, re: &Regex) -> Result<Op> {
    let caps = re.captures(&line).ok_or("parse error")?;
    Ok((
        caps[1].to_string(),
        caps[2].to_string(),
        caps[3].parse::<isize>().map_err(|e| e.to_string())?,
        caps[4].to_string(),
        caps[5].to_string(),
        caps[6].parse::<isize>().map_err(|e| e.to_string())?,
    ))
}

fn eval_exp(a: isize, oper: &str, b: isize) -> Result<bool> {
    let result = match oper {
        "==" => a == b,
        "!=" => a != b,
        "<" => a < b,
        ">" => a > b,
        "<=" => a <= b,
        ">=" => a >= b,
        _ => return Err(format!("unknown operator: {}", oper)),
    };
    Ok(result)
}

fn execute<T>(ops: T) -> Result<(HashMap<String, isize>, isize)>
where
    T: IntoIterator<Item = Op>,
{
    let mut regs = HashMap::<String, isize>::new();
    let mut all_time_max = 0;
    for op in ops {
        let a = *regs.get(&op.3).unwrap_or(&0);
        if eval_exp(a, &op.4, op.5)? {
            if !regs.contains_key(&op.0) {
                regs.insert(op.0.to_string(), 0);
            }
            let reg = regs.get_mut(&op.0).ok_or("impossible None error")?;
            *reg = match op.1.as_str() {
                "inc" => *reg + op.2,
                "dec" => *reg - op.2,
                x => return Err(format!("unsupported operator: {}", x)),
            };
            all_time_max = std::cmp::max(all_time_max, *reg);
        }
    }
    Ok((regs, all_time_max))
}

fn solve<T>(ops: T) -> Result<(isize, isize)>
where
    T: IntoIterator<Item = Op>,
{
    let (regs, all_time_max) = execute(ops)?;
    let part1 = *regs.values().max().ok_or("max not found")?;
    Ok((part1, all_time_max))
}

fn parse_input(fname: &str, re: &Regex) -> Result<Vec<Op>> {
    let mut result = vec![];
    let file = File::open(fname).map_err(|e| e.to_string())?;
    for line in BufReader::new(&file).lines() {
        let op = line
            .map_err(|e| e.to_string())
            .and_then(|l| parse_line(l, re))?;
        result.push(op);
    }
    Ok(result)
}

fn get_re() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (==|!=|<|>|<=|>=) (-?\d+)$")
                .unwrap();
    }
    &RE
}

fn main() {
    match parse_input("resources/day8_input.txt", get_re()).and_then(solve) {
        Ok((part1, part2)) => {
            println!("part 1: {}", part1);
            println!("part 2: {}", part2);
        }
        Err(e) => eprintln!("{}", e),
    }
}

mod tests {

    #[test]
    fn solve_test() {
        use super::*;
        let input = parse_input("resources/day8_testdata.txt", get_re()).unwrap();
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(1, part1);
        assert_eq!(10, part2);
    }
}
