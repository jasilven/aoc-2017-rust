use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

fn parse_input(fname: &str) -> Result<Vec<String>> {
    let file = File::open(fname).map_err(|e| e.to_string())?;
    BufReader::new(file)
        .lines()
        .map(|line| line.map_err(|e| e.to_string()))
        .collect()
}

fn solve1<T>(lines: T) -> usize
where
    T: IntoIterator<Item = String>,
{
    let mut result = 0;
    for line in lines {
        let mut words: Vec<&str> = line.split(' ').collect();
        let old_len = words.len();
        words.sort();
        words.dedup();
        if words.len() == old_len {
            result += 1;
        }
    }
    result
}

fn solve2<T>(lines: T) -> usize
where
    T: IntoIterator<Item = String>,
{
    let mut result = 0;
    for line in lines {
        let mut words: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        for word in words.iter_mut() {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let sorted = chars.iter().collect::<String>();
            *word = sorted.clone();
        }
        words.sort();
        let old_len = words.len();
        words.dedup();
        if old_len == words.len() {
            result += 1;
        }
    }
    result
}

fn main() -> Result<()> {
    let lines = parse_input("resources/day4_input.txt")?;
    println!("part 1: {}", solve1(lines.clone()));
    println!("part 2: {}", solve2(lines));
    Ok(())
}

mod tests {

    #[test]
    fn test_part1() {
        use super::{parse_input, solve1};
        let lines = parse_input("resources/day4_test.txt").unwrap();
        assert_eq!(2, solve1(lines));
    }
}
