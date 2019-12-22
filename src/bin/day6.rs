use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

fn parse_input(fname: &str) -> Vec<usize> {
    let line = read_to_string(fname).expect("unable to open file");
    line.split_whitespace()
        .map(|s| usize::from_str_radix(s, 10).expect("unable to parse number"))
        .collect()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn find_target_bank(banks: &[usize]) -> (usize, usize) {
    let (max_bank, blocks) = banks
        .iter()
        .enumerate()
        .max_by(|(i, x), (j, y)| if x == y { j.cmp(i) } else { x.cmp(y) })
        .expect("unable to find target bank");
    (max_bank, *blocks)
}

#[allow(unused_assignments)]
fn solve1(banks: &[usize]) -> (usize, Vec<usize>) {
    let mut banks = banks.to_vec();
    let mut cycles = 0;
    let mut seen = HashSet::new();
    let mut seen_banks = Vec::<usize>::new();

    seen.insert(calculate_hash(&banks));
    loop {
        let (max_bank, blocks) = find_target_bank(&banks);
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
    (cycles, seen_banks)
}

fn solve2(banks: &[usize]) -> usize {
    let (_, seen_banks) = solve1(banks);
    solve1(&seen_banks).0
}

fn main() {
    let banks = parse_input("resources/day6_input.txt");
    println!("part 1: {}", solve1(&banks).0);
    println!("part 2: {}", solve2(&banks));
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let banks = parse_input("resources/day6_testdata.txt");
        assert_eq!(5, solve1(&banks).0);
    }

    #[test]
    fn test_part2() {
        let banks = parse_input("resources/day6_testdata.txt");
        assert_eq!(4, solve2(&banks));
    }
}
