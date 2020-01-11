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

fn solve1(numbers: Vec<usize>, lengths: &[usize]) -> Vec<usize> {
    let mut numbers = numbers;
    let numbers_len = numbers.len();
    let mut rotations = 0;
    for (skip_size, length) in lengths.iter().enumerate() {
        let (l, r) = numbers.split_at(*length);
        let mut left = l.to_vec();
        left.reverse();
        left.extend_from_slice(&r);
        numbers = left;
        let curpos = (skip_size + *length) % numbers_len;
        numbers.rotate_left(curpos);
        rotations += curpos;
    }
    numbers.rotate_right(rotations % numbers_len);
    numbers
}

fn parse_lengths2(fname: &str) -> Result<Vec<usize>> {
    let bs = fs::read_to_string(fname)
        .map_err(|e| e.to_string())?
        .trim()
        .bytes()
        .map(|b| b as usize)
        .collect();
    Ok(bs)
}

// fn solve2(numbers: Vec<usize>, lengths: &[usize]) -> Vec<usize> {
//     for _ in 0..64 {

//     }

// }

fn main() -> Result<()> {
    let numbers: Vec<usize> = (0..=255).collect();
    let lengths = parse_lengths("resources/day10_input.txt")?;
    let mut lengths2 = parse_lengths2("resources/day10_input.txt")?;
    lengths2.extend_from_slice(&[17, 31, 73, 47, 23]);
    let result1 = solve1(numbers.clone(), &lengths);
    println!("part 1: {:?}", result1[0] * result1[1]);
    Ok(())
}

mod tests {
    #[test]
    fn test_solve() {
        use super::*;
        assert_eq!(
            vec![3, 4, 2, 1, 0],
            solve1(vec![0, 1, 2, 3, 4], &[3, 4, 1, 5])
        );
    }
}
