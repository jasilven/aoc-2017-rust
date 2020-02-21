use failure::{bail, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone, Debug)]
struct Virus {
    position: (isize, isize),
    direction: Direction,
}

fn parse_infections(fname: &str) -> Result<HashMap<(isize, isize), Status>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut result = HashMap::new();

    for (y, line) in reader.lines().enumerate() {
        for (x, ch) in line?.chars().enumerate() {
            match ch {
                '#' => {
                    let _ = result.insert((x as isize, y as isize), Status::Infected);
                }
                '.' => {
                    let _ = result.insert((x as isize, y as isize), Status::Clean);
                }
                _ => bail!("Parse error"),
            }
        }
    }

    Ok(result)
}

impl Virus {
    fn new(position: (isize, isize), direction: Direction) -> Self {
        Virus {
            position,
            direction,
        }
    }

    fn burst(&mut self, infections: &mut HashMap<(isize, isize), Status>) -> bool {
        use Direction::*;
        let mut infection = false;

        if let Some(Status::Infected) = infections.get(&self.position) {
            self.turn_right();
            infections.insert(self.position, Status::Clean);
        } else {
            self.turn_left();
            infections.insert(self.position, Status::Infected);
            infection = true;
        }

        self.position = match self.direction {
            Up => (self.position.0, self.position.1 - 1),
            Right => (self.position.0 + 1, self.position.1),
            Down => (self.position.0, self.position.1 + 1),
            Left => (self.position.0 - 1, self.position.1),
        };

        infection
    }

    fn burst2(&mut self, infections: &mut HashMap<(isize, isize), Status>) -> bool {
        use Direction::*;
        use Status::*;

        let mut infection = false;

        match infections.get(&self.position) {
            Some(Weakened) => {
                infections.insert(self.position, Infected);
                infection = true;
            }
            Some(Infected) => {
                infections.insert(self.position, Flagged);
                self.turn_right();
            }
            Some(Flagged) => {
                infections.insert(self.position, Clean);
                self.turn_right();
                self.turn_right();
            }
            Some(Clean) | _ => {
                infections.insert(self.position, Weakened);
                self.turn_left();
            }
        }

        self.position = match self.direction {
            Up => (self.position.0, self.position.1 - 1),
            Right => (self.position.0 + 1, self.position.1),
            Down => (self.position.0, self.position.1 + 1),
            Left => (self.position.0 - 1, self.position.1),
        };

        infection
    }

    fn turn_left(&mut self) {
        use Direction::*;

        self.direction = match self.direction {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        };
    }

    fn turn_right(&mut self) {
        use Direction::*;

        self.direction = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
    }
}

fn solve1(mut infections: HashMap<(isize, isize), Status>, mut virus: Virus) -> isize {
    let mut result = 0;

    for _ in 0..10000 {
        if virus.burst(&mut infections) {
            result += 1;
        }
    }

    result
}

fn solve2(mut infections: HashMap<(isize, isize), Status>, mut virus: Virus) -> isize {
    let mut result = 0;

    for _ in 0..10_000_000 {
        if virus.burst2(&mut infections) {
            result += 1;
        }
    }

    result
}

fn main() -> Result<()> {
    let infections = parse_infections("resources/day22_input.txt")?;
    let virus = Virus::new((12, 12), Direction::Up);

    println!("part 1: {}", solve1(infections.clone(), virus.clone()));
    println!("part 2: {}", solve2(infections, virus));

    Ok(())
}
