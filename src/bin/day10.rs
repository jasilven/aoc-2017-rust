use std::fs;

mod knothash;

type Result<T> = std::result::Result<T, String>;

fn parse_lengths(fname: &str) -> Result<Vec<usize>> {
    fs::read_to_string(fname)
        .map_err(|e| e.to_string())?
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
        .collect()
}

fn solve1(input: &[usize]) -> Vec<usize> {
    let (result, _) = knothash::hash(input, 1);
    result
}

fn parse_lengths2(fname: &str) -> Result<Vec<usize>> {
    fs::read_to_string(fname)
        .map_err(|e| e.to_string())?
        .trim()
        .bytes()
        .map(|b| Ok(b as usize))
        .collect()
}

fn solve2(input: &[usize]) -> String {
    let mut lengths = input.to_vec();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let (_, result) = knothash::hash(&lengths, 64);
    result
}

fn main() -> Result<()> {
    let input_file = "resources/day10_input.txt";
    let result1 = solve1(&parse_lengths(input_file)?);
    let result2 = solve2(&parse_lengths2(input_file)?);
    println!("part 1: {}", result1[0] * result1[1]);
    println!("part 2: {}", result2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve2() {
        let result = solve2(&[]);
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", result);
    }

    #[test]
    fn test_bitxor() {
        let numbers = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        let bxor = numbers.iter().fold(0, |acc, n| acc ^ n);
        assert_eq!(64, bxor);
    }
}
