use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

fn parse_input(fname: &str) -> Result<Vec<isize>> {
    let file = File::open(fname).map_err(|e| e.to_string())?;
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_else(|e| e.to_string()))
        .map(|line| line.parse::<isize>().map_err(|e| e.to_string()))
        .collect()
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

fn main() -> Result<()> {
    let input = parse_input("resources/day5_input.txt")?;
    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
    Ok(())
}

mod tests {

    #[test]
    fn test_part1() {
        use super::*;
        let input = parse_input("resources/day5_testdata.txt").unwrap();
        assert_eq!(5, solve1(&input));
    }

    #[test]
    fn test_part2() {
        use super::*;
        let input = parse_input("resources/day5_testdata.txt").unwrap();
        assert_eq!(10, solve2(&input));
    }
}
