use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
struct Scanner {
    range: usize,
    pos: usize,
}

fn parse_scanners(fname: &str) -> Result<HashMap<usize, Scanner>> {
    let mut result = HashMap::<usize, Scanner>::new();
    let f = File::open(fname).map_err(|e| e.to_string())?;
    for line in BufReader::new(f).lines() {
        let line = line.map_err(|e| e.to_string())?;
        let sp: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        let depth = sp[0].parse::<usize>().map_err(|e| e.to_string())?;
        let range = sp[1].parse::<usize>().map_err(|e| e.to_string())?;

        result.insert(depth, Scanner { range, pos: 0 });
    }
    Ok(result)
}

fn reset_scanners(scanners: &mut HashMap<usize, Scanner>) {
    for (_, scanner) in scanners.iter_mut() {
        scanner.pos = 0;
    }
}

fn move_scanners(tick: usize, scanners: &mut HashMap<usize, Scanner>) -> Result<()> {
    for (_, scanner) in scanners.iter_mut() {
        if (tick / (scanner.range - 1) % 2) == 0 {
            scanner.pos = (scanner.pos + 1) % scanner.range;
        } else {
            scanner.pos = (scanner.pos - 1) % scanner.range;
        }
    }
    Ok(())
}

fn layers_cnt(scanners: &HashMap<usize, Scanner>) -> Result<usize> {
    let result: usize = *scanners
        .keys()
        .max()
        .ok_or_else(|| String::from("0 layers"))?;
    Ok(result)
}

fn solve1(tick: usize, scanners: &mut HashMap<usize, Scanner>) -> Result<(usize, bool)> {
    let mut severity = 0;
    let mut caught = false;
    let layers_cnt = layers_cnt(&scanners)?;
    let mut tick = tick;
    let mut packet_pos = 0;

    while packet_pos <= layers_cnt {
        if let Some(scanner) = scanners.get(&packet_pos) {
            if scanner.pos == 0 {
                severity += packet_pos * scanner.range;
                caught = true;
            }
        }
        move_scanners(tick, scanners)?;
        packet_pos += 1;
        tick += 1;
    }

    Ok((severity, caught))
}

fn solve2(scanners: &mut HashMap<usize, Scanner>) -> Result<usize> {
    let mut result = 0;
    let layers_cnt = layers_cnt(&scanners)?;

    for delay in 0.. {
        let mut caught = false;
        for tick in 0..=layers_cnt {
            if let Some(scanner) = scanners.get(&tick) {
                if (delay + tick) % (2 * scanner.range - 2) == 0 {
                    caught = true;
                    break;
                }
            }
        }
        if !caught {
            result = delay;
            break;
        } else {
            continue;
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    let mut scanners = parse_scanners("resources/day13_input.txt")?;
    let (severity, _) = solve1(0, &mut scanners)?;
    println!("part 1: {}", severity);

    reset_scanners(&mut scanners);
    println!("part 2: {:?}", solve2(&mut scanners)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut scanners = parse_scanners("resources/day13_testdata.txt").unwrap();
        let (severity, _) = solve1(0, &mut scanners).unwrap();
        assert_eq!(24, severity);
    }

    #[test]
    fn test_part2() {
        let mut scanners = parse_scanners("resources/day13_testdata.txt").unwrap();
        assert_eq!(10, solve2(&mut scanners).unwrap())
    }
}
