use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};

fn parse_map(
    fname: &str,
) -> Result<
    (
        HashSet<(isize, isize)>,
        HashMap<(isize, isize), char>,
        (isize, isize),
    ),
    String,
> {
    let file = fs::File::open(fname).map_err(|_| "File open error".to_string())?;
    let mut map = HashSet::<(isize, isize)>::new();
    let mut letters = HashMap::<(isize, isize), char>::new();
    let mut start = (0isize, 0isize);

    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|_| "Parse error".to_string())?;

        if y == 0 {
            start = match line.chars().enumerate().find(|(_, ch)| ch == &'|') {
                Some((x, _)) => (x as isize, 0),
                _ => return Err("Starting position not found".into()),
            };
        }

        for (x, ch) in line.chars().enumerate() {
            if ch != ' ' {
                map.insert((x as isize, y as isize));
                if ch.is_ascii_alphabetic() {
                    letters.insert((x as isize, y as isize), ch);
                }
            }
        }
    }

    Ok((map, letters, start))
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Walker {
    position: (isize, isize),
    direction: Direction,
    map: HashSet<(isize, isize)>,
}

impl Walker {
    fn new(start: (isize, isize), direction: Direction, map: HashSet<(isize, isize)>) -> Self {
        Walker {
            position: start,
            direction,
            map,
        }
    }

    fn walk(&mut self) -> Option<(isize, isize)> {
        if let Some(pos) = self.forward() {
            self.position = *pos;
            Some(self.position)
        } else if let Some(pos) = self.left() {
            self.position = *pos;
            self.direction = match self.direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            };
            Some(self.position)
        } else if let Some(pos) = self.right() {
            self.position = *pos;
            self.direction = match self.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            Some(self.position)
        } else {
            None
        }
    }

    fn forward(&self) -> Option<&(isize, isize)> {
        match self.direction {
            Direction::Up => self.map.get(&(self.position.0, self.position.1 - 1)),
            Direction::Right => self.map.get(&(self.position.0 + 1, self.position.1)),
            Direction::Down => self.map.get(&(self.position.0, self.position.1 + 1)),
            Direction::Left => self.map.get(&(self.position.0 - 1, self.position.1)),
        }
    }

    fn left(&self) -> Option<&(isize, isize)> {
        match self.direction {
            Direction::Up => self.map.get(&(self.position.0 - 1, self.position.1)),
            Direction::Right => self.map.get(&(self.position.0, self.position.1 - 1)),
            Direction::Down => self.map.get(&(self.position.0 + 1, self.position.1)),
            Direction::Left => self.map.get(&(self.position.0, self.position.1 + 1)),
        }
    }

    fn right(&self) -> Option<&(isize, isize)> {
        match self.direction {
            Direction::Down => self.map.get(&(self.position.0 - 1, self.position.1)),
            Direction::Left => self.map.get(&(self.position.0, self.position.1 - 1)),
            Direction::Up => self.map.get(&(self.position.0 + 1, self.position.1)),
            Direction::Right => self.map.get(&(self.position.0, self.position.1 + 1)),
        }
    }
}

fn solve(
    map: HashSet<(isize, isize)>,
    letters: &HashMap<(isize, isize), char>,
    start: (isize, isize),
) -> (String, isize) {
    let mut result = String::from("");
    let mut steps = 1;

    let mut walker = Walker::new(start, Direction::Down, map);

    while let Some(pos) = walker.walk() {
        steps += 1;
        if let Some(ch) = letters.get(&pos) {
            result.push(*ch)
        }
    }

    (result, steps)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (map, letters, start) = parse_map("resources/day19_input.txt")?;

    let (part1, part2) = solve(map, &letters, start);
    println!("part 1: {}", &part1);
    println!("part 2: {}", &part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (map, letters, start) = parse_map("resources/day19_testdata.txt").unwrap();
        let (part1, steps) = solve(map, &letters, start);
        assert_eq!("ABCDEF", part1);
        assert_eq!(38, steps);
    }
}
