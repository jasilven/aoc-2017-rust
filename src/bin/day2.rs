use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input(path: &str) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let reader = BufReader::new(File::open(&path).expect("failed to open file"));

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let mut nums: Vec<usize> = line
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().expect("parse error"))
            .collect();
        nums.sort();
        result.push(nums);
    }
    result
}

fn solve1(sorted_nums: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for row_nums in sorted_nums {
        let first = row_nums.first().expect("first not found");
        let last = row_nums.last().expect("last not found");
        result += first.max(&last) - first.min(&last);
    }
    result
}

fn solve2(sorted_nums: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for row_nums in sorted_nums {
        for i in 0..(row_nums.len() - 1) {
            let smaller = row_nums[i];
            for bigger in &row_nums[(i + 1)..] {
                if (bigger % smaller) == 0 {
                    result += bigger / smaller;
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    let sorted_nums = parse_input("resources/day2_input.csv");
    println!("Part 1: {}", solve1(&sorted_nums));
    println!("Part 2: {}", solve2(&sorted_nums));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_solve1() {
        use super::*;
        let data = parse_input("resources/day2_testdata.csv");
        assert_eq!(18, solve1(&data));
    }

    #[test]
    fn test_solve2() {
        use super::*;
        let data = parse_input("resources/day2_testdata2.csv");
        assert_eq!(9, solve2(&data));
    }
}
