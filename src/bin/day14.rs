use std::collections::HashSet;
use std::collections::VecDeque;

mod knothash;

type Result<T> = std::result::Result<T, String>;

fn hex2bin(input: &str) -> Result<String> {
    let mut result = String::from("");

    for ch in input.trim().chars() {
        let i: u32 = ch.to_digit(16).ok_or_else(|| String::from("parse error"))?;
        result.push_str(&format!("{:04b}", i));
    }
    Ok(result)
}

fn build_grid(key: &str) -> Result<HashSet<(isize, isize)>> {
    let mut result = HashSet::<_, _>::new();

    for i in 0..128 {
        let key = format!("{}-{}", key, i);

        let mut hash_input: Vec<usize> = key.bytes().map(|b| b as usize).collect();
        hash_input.extend_from_slice(&[17, 31, 73, 47, 23]);
        let (_, hash) = knothash::hash(&hash_input, 64);
        let bin = hex2bin(&hash)?;
        bin.chars().enumerate().for_each(|(j, ch)| {
            if ch == '1' {
                result.insert((j as isize, i as isize));
            }
        });
    }
    Ok(result)
}

fn adjacents(point: &(isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
        (point.0, point.1 + 1),
    ]
}

fn solve2(grid: &HashSet<(isize, isize)>) -> Result<usize> {
    let mut seen = HashSet::<(isize, isize)>::new();
    let mut regions = vec![];

    for key in grid.iter().filter(|k| grid.get(k).is_some()) {
        if seen.contains(key) {
            continue;
        }

        let mut region = vec![];
        let mut stack: VecDeque<(isize, isize)> = VecDeque::new();
        stack.push_back(*key);

        while !stack.is_empty() {
            let cur = stack.pop_front().unwrap();
            seen.insert(cur);
            region.push(cur);

            for adj in adjacents(&cur).iter() {
                if grid.get(adj).is_some() && !seen.contains(adj) {
                    stack.push_back(*adj);
                }
            }
        }
        regions.push(region);
    }

    Ok(regions.len())
}

fn main() -> Result<()> {
    let grid = build_grid("uugsqrei")?;
    println!("part 1: {}", grid.len());
    println!("part 2: {}", solve2(&grid)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex2bin() {
        assert_eq!("0001", &hex2bin("1").unwrap());
        assert_eq!("0000", &hex2bin("0").unwrap());
        assert_eq!("1110", &hex2bin("e").unwrap());
        assert_eq!("1111", &hex2bin("f").unwrap());
        assert_eq!("0111", &hex2bin("7").unwrap());
        assert_eq!("11010100111101110110101111011100101111111000001110001111100001000001011011001100111110101000101111000110110100011111100111100110", &hex2bin("d4f76bdcbf838f8416ccfa8bc6d1f9e6").unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(8108, build_grid("flqrgnkx").unwrap().len());
    }
}
