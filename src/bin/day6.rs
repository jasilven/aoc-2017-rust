use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

type Result<T> = std::result::Result<T, String>;

fn parse_input(fname: &str) -> Result<Vec<usize>> {
    read_to_string(fname)
        .map_err(|e| e.to_string())?
        .split_whitespace()
        .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
        .collect()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn find_target_bank(banks: &[usize]) -> Result<(usize, usize)> {
    banks
        .iter()
        .enumerate()
        .max_by(|(i, x), (j, y)| if x == y { j.cmp(i) } else { x.cmp(y) })
        .ok_or_else(|| "max not found".to_string())
        .map(|(i, b)| Ok((i, *b)))?
}

#[allow(unused_assignments)]
fn solve1(banks: &[usize]) -> Result<(usize, Vec<usize>)> {
    let mut banks = banks.to_vec();
    let mut cycles = 0;
    let mut seen = HashSet::new();
    let mut seen_banks = Vec::<usize>::new();

    seen.insert(calculate_hash(&banks));
    loop {
        let (max_bank, blocks) = find_target_bank(&banks)?;
        banks[max_bank] = 0;
        for i in 1..=blocks {
            let index = (i + max_bank) % banks.len();
            banks[index] += 1;
        }
        cycles += 1;
        if !seen.insert(calculate_hash(&banks)) {
            seen_banks = banks.to_vec();
            break;
        }
    }
    Ok((cycles, seen_banks))
}

fn solve2(banks: &[usize]) -> Result<usize> {
    let (_, seen_banks) = solve1(banks)?;
    Ok(solve1(&seen_banks)?.0)
}

fn main() -> Result<()> {
    let banks = parse_input("resources/day6_input.txt")?;
    println!("part 1: {}", solve1(&banks)?.0);
    println!("part 2: {}", solve2(&banks)?);
    Ok(())
}

mod tests {

    #[test]
    fn test_part1() {
        use super::*;
        let banks = parse_input("resources/day6_testdata.txt").unwrap();
        assert_eq!(5, solve1(&banks).unwrap().0);
    }

    #[test]
    fn test_part2() {
        use super::*;
        let banks = parse_input("resources/day6_testdata.txt").unwrap();
        assert_eq!(4, solve2(&banks).unwrap());
    }
}
