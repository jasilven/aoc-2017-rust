use std::fs;

type Result<T> = std::result::Result<T, String>;

fn parse_lengths(fname: &str) -> Result<Vec<usize>> {
    fs::read_to_string(fname)
        .map_err(|e| e.to_string())?
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
        .collect()
}

fn solve1(numbers: &[usize], lengths: &[usize], rounds: usize) -> Vec<usize> {
    let mut numbers = numbers.to_vec();
    let numbers_len = numbers.len();
    let mut skip_size = 0;
    let mut rotations = 0;
    for _ in 0..rounds {
        for length in lengths.iter() {
            let (l, r) = numbers.split_at(*length);
            let mut left = l.to_vec();
            left.reverse();
            left.extend_from_slice(&r);
            numbers = left;
            let curpos = (skip_size + *length) % numbers_len;
            numbers.rotate_left(curpos);
            rotations += curpos;
            skip_size += 1;
        }
    }
    numbers.rotate_right(rotations % numbers_len);
    numbers
}

fn parse_lengths2(fname: &str) -> Result<Vec<usize>> {
    fs::read_to_string(fname)
        .map_err(|e| e.to_string())?
        .trim()
        .bytes()
        .map(|b| Ok(b as usize))
        .collect()
}

fn dense_hash(numbers: &[usize]) -> String {
    let mut result = String::from("");
    for chunk in numbers.chunks(16) {
        let mut chunk_iter = chunk.iter();
        let first = *chunk_iter.next().unwrap();
        let bxor = chunk_iter.fold(first, |acc, n| acc ^ n);
        result.push_str(&format!("{:02x}", bxor));
    }
    result
}

fn solve2(numbers: &[usize], lengths: &[usize]) -> String {
    let mut lengths = lengths.to_vec();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let nums = solve1(&numbers, &lengths, 64);
    dense_hash(&nums)
}

fn main() -> Result<()> {
    let input_file = "resources/day10_input.txt";
    let numbers: Vec<usize> = (0..=255).collect();
    let result1 = solve1(&numbers, &parse_lengths(input_file)?, 1);
    let result2 = solve2(&numbers, &parse_lengths2(input_file)?);
    println!("part 1: {}", result1[0] * result1[1]);
    println!("part 2: {}", result2);
    Ok(())
}

mod tests {
    #[test]
    fn test_solve1() {
        use super::*;
        let result = solve1(&[0, 1, 2, 3, 4], &[3, 4, 1, 5], 1);
        assert_eq!(result, vec![3, 4, 2, 1, 0],);
    }

    #[test]
    fn test_solve2() {
        use super::*;
        let numbers: Vec<usize> = (0..=255).collect();
        let result = solve2(&numbers, &[]);
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", result);
    }

    #[test]
    fn test_bitxor() {
        let numbers = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        let bxor = numbers.iter().fold(0, |acc, n| acc ^ n);
        assert_eq!(64, bxor);
    }

    #[test]
    fn test_dense_hash1() {
        use super::*;
        let numbers = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!("40", dense_hash(&numbers));
    }
}
