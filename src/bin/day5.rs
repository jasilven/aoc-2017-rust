use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(fname: &str) -> Vec<isize> {
    let file = File::open(fname).expect("unable to open file");
    let mut lines: Vec<isize> = vec![];
    for line in BufReader::new(file).lines() {
        let line = line.expect("unable to read line");
        let num = isize::from_str_radix(&line, 10).expect("unable to parse integer");
        lines.push(num);
    }
    lines
}

fn solve1(nums: &[isize]) -> isize {
    let mut nums = nums.to_vec();
    let mut pc = 0isize;
    let mut result = 0;

    loop {
        if pc < 0 || pc >= nums.len() as isize {
            break;
        }
        result += 1;
        let old_pc = pc;
        pc += nums[pc as usize];
        nums[old_pc as usize] += 1;
    }
    result
}

fn solve2(nums: &[isize]) -> isize {
    let mut nums = nums.to_vec();
    let mut pc = 0isize;
    let mut result = 0;

    loop {
        if pc < 0 || pc >= nums.len() as isize {
            break;
        }
        result += 1;
        let old_pc = pc;
        pc += nums[pc as usize];

        if nums[old_pc as usize] >= 3isize {
            nums[old_pc as usize] -= 1;
        } else {
            nums[old_pc as usize] += 1;
        }
    }
    result
}

fn main() {
    let input = parse_input("resources/day5_input.txt");
    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("resources/day5_testdata.txt");
        assert_eq!(5, solve1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input("resources/day5_testdata.txt");
        assert_eq!(10, solve2(&input));
    }
}
