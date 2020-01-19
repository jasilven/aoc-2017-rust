use std::thread;

fn solve1(seed_a: u64, seed_b: u64, cnt: u64) -> u64 {
    let div = 2_147_483_647;
    let mut result = 0;
    let mut a = seed_a;
    let mut b = seed_b;

    for _ in 0..cnt {
        a = (16807 * a) % div;
        b = (48271 * b) % div;
        if (a as u16) == (b as u16) {
            result += 1;
        }
    }

    result
}

fn solve2(prev_a: u64, prev_b: u64, cnt: u64) -> u64 {
    let div = 2_147_483_647;
    let mut result = 0;
    let mut a = prev_a;
    let mut b = prev_b;

    for _ in 0..cnt {
        let a_handle = thread::spawn(move || loop {
            a = (16807 * a) % div;
            if a % 4 == 0 {
                return a;
            }
        });

        loop {
            b = (48271 * b) % div;
            if b % 8 == 0 {
                break;
            }
        }
        a = a_handle.join().unwrap();
        if (a as u16) == (b as u16) {
            result += 1;
        }
    }
    result
}

fn main() {
    println!("part 1: {}", solve1(277, 349, 40_000_000));
    println!("part 2: {}", solve2(277, 349, 5_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(1, solve1(65, 8921, 5));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(309, solve2(65, 8921, 5_000_000));
    }
}
