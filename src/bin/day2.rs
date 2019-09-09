use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input(path: &str) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let reader = BufReader::new(File::open(&path).expect("failed to open file"));

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let mut nums: Vec<usize> = line
            .split(',')
            .map(|s| s.parse::<usize>().expect("parse error"))
            .collect();
        nums.sort();
        result.push(nums);
    }
    result
}

fn solve1(nums: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for row_nums in nums {
        let first = row_nums.first().expect("first not found");
        let last = row_nums.last().expect("last not found");
        result += first.max(&last) - first.min(&last);
    }
    result
}

fn main() {
    let data = parse_input("resources/day2_input.csv");
    println!("Part 1: {}", solve1(&data));
}
