use failure::bail;
use failure::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, Error>;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

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
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0 + 1, self.position.1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
        };

        infection
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
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

fn main() -> Result<()> {
    let infections = parse_infections("resources/day22_input.txt")?;
    let virus = Virus::new((12, 12), Direction::Up);

    println!("part 1: {}", solve1(infections, virus));

    Ok(())
}
