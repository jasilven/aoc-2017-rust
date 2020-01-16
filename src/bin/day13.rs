use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

fn parse_scanners(fname: &str) -> Result<HashMap<usize, usize>> {
    let mut result = HashMap::<usize, usize>::new();
    let f = File::open(fname).map_err(|e| e.to_string())?;
    for line in BufReader::new(f).lines() {
        let line = line.map_err(|e| e.to_string())?;
        let sp: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        let depth = sp[0].parse::<usize>().map_err(|e| e.to_string())?;
        let range = sp[1].parse::<usize>().map_err(|e| e.to_string())?;

        result.insert(depth, range);
    }
    Ok(result)
}

#[allow(dead_code)]
fn move_scanners(
    tick: usize,
    scanner_pos: &mut HashMap<usize, usize>,
    scanners: &HashMap<usize, usize>,
) -> Result<()> {
    for (scanner, pos) in scanner_pos.iter_mut() {
        let range = scanners
            .get(scanner)
            .ok_or_else(|| format!("scanner range not found: {}", scanner))?;
        let up = (tick / (range - 1) % 2) == 0;
        if up {
            *pos = (*pos + 1) % range;
        } else {
            *pos = (*pos - 1) % range;
        }
    }
    Ok(())
}

fn solve(scanners: &HashMap<usize, usize>) -> Result<usize> {
    let mut packet_pos = 0;
    let mut scanner_pos = HashMap::<usize, usize>::new();
    let layers_cnt: usize = *scanners.keys().max().unwrap_or(&0);
    let mut severity = 0;

    for scanner in scanners.keys() {
        scanner_pos.insert(*scanner, 0);
    }

    let mut tick = 0;
    while packet_pos <= layers_cnt {
        if let Some(&0) = scanner_pos.get(&packet_pos) {
            if let Some(depth) = scanners.get(&packet_pos) {
                severity += packet_pos * depth;
            }
        }
        move_scanners(tick, &mut scanner_pos, &scanners)?;
        packet_pos += 1;
        tick += 1;
    }

    Ok(severity)
}

fn main() -> Result<()> {
    let scanners = parse_scanners("resources/day13_input.txt")?;
    let part1 = solve(&scanners)?;
    println!("part1: {:?}", part1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let scanners = parse_scanners("resources/day13_testdata.txt").unwrap();
        assert_eq!(24, solve(&scanners).unwrap())
    }
}
