use std::fs::read_to_string;

type Result<T> = std::result::Result<T, String>;

fn parse_input(path: &str) -> Result<Vec<u8>> {
    read_to_string(&path)
        .map_err(|e| e.to_string())?
        .chars()
        .map(|ch| ch.to_digit(10).ok_or_else(|| "parse error".to_string()))
        .map(|digit| Ok(digit.unwrap() as u8))
        .collect()
}

fn solve1(digits: &[u8]) -> usize {
    let mut result = 0;
    let mut digits1: Vec<u8> = digits.to_vec();
    digits1.push(digits[0]);
    for window in digits1.windows(2) {
        if window[0] == window[1] {
            result += window[0] as usize;
        }
    }
    result
}

fn solve2(digits: &[u8]) -> usize {
    let mut result = 0;
    let cnt = digits.len();
    let offset = cnt / 2;
    for (index, digit) in digits.iter().enumerate() {
        if digits[(index + offset) % cnt] == *digit {
            result += *digit as usize;
        }
    }
    result
}

fn main() -> Result<()> {
    let digits = parse_input("resources/day1_input.txt")?;
    println!("Part 1: {}", solve1(&digits));
    println!("Part 2: {}", solve2(&digits));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_solve1() {
        use super::*;
        let digits1 = vec![1, 1, 2, 2];
        let digits2 = vec![1, 1, 1, 1];
        let digits3 = vec![1, 2, 3, 4];
        assert_eq!(3, solve1(&digits1));
        assert_eq!(4, solve1(&digits2));
        assert_eq!(0, solve1(&digits3));
    }

    #[test]
    fn test_solve2() {
        use super::*;
        let digits1 = vec![1, 2, 1, 2];
        let digits2 = vec![1, 2, 2, 1];
        let digits3 = vec![1, 2, 3, 4, 2, 5];
        let digits4 = vec![1, 2, 3, 1, 2, 3];
        let digits5 = vec![1, 2, 1, 3, 1, 4, 1, 5];
        assert_eq!(6, solve2(&digits1));
        assert_eq!(0, solve2(&digits2));
        assert_eq!(4, solve2(&digits3));
        assert_eq!(12, solve2(&digits4));
        assert_eq!(4, solve2(&digits5));
    }
}
