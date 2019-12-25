use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

fn parse_input(path: &str) -> Result<Vec<Vec<usize>>> {
    let mut result = vec![];
    let file = File::open(path).map_err(|e| e.to_string())?;
    for line in BufReader::new(file).lines() {
        let mut nums: Vec<usize> = vec![];
        for s in line.map_err(|e| e.to_string())?.trim().split(',') {
            let n = s.parse::<usize>().map_err(|e| e.to_string())?;
            nums.push(n);
        }
        nums.sort();
        result.push(nums);
    }
    Ok(result)
}

fn solve1(sorted_nums: &[Vec<usize>]) -> Result<usize> {
    let mut result = 0;
    for row_nums in sorted_nums {
        let first = row_nums
            .first()
            .ok_or_else(|| "first not found".to_owned())?;
        let last = row_nums.last().ok_or_else(|| "last not found".to_owned())?;
        result += first.max(&last) - first.min(&last);
    }
    Ok(result)
}

fn solve2(sorted_nums: &[Vec<usize>]) -> usize {
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

fn main() -> Result<()> {
    let sorted_nums = parse_input("resources/day2_input.csv")?;
    println!("Part 1: {}", solve1(&sorted_nums)?);
    println!("Part 2: {}", solve2(&sorted_nums));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_solve1() {
        use super::*;
        let data = parse_input("resources/day2_testdata.csv").unwrap();
        assert_eq!(18, solve1(&data).unwrap());
    }

    #[test]
    fn test_solve2() {
        use super::*;
        let data = parse_input("resources/day2_testdata2.csv").unwrap();
        assert_eq!(9, solve2(&data));
    }
}
