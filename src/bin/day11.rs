use std::fs;

type Result<T> = std::result::Result<T, String>;

struct Position(isize, isize, isize);

impl Position {
    fn origin() -> Position {
        Position(0, 0, 0)
    }
    // distance in hex grid is half of manhattan distance
    fn distance(&self, other: &Position) -> usize {
        ((isize::abs(self.0 - other.0)
            + isize::abs(self.1 - other.1)
            + isize::abs(self.2 - other.2))
            / 2) as usize
    }
}

fn parse_directions(fname: &str) -> Result<Vec<String>> {
    let dirs = fs::read_to_string(fname)
        .map_err(|e| e.to_string())?
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    Ok(dirs)
}

fn solve(directions: &[String]) -> Result<(usize, usize)> {
    let mut position = Position::origin();
    let mut max_distance = 0;
    let mut distance = 0;
    for direction in directions {
        match direction.as_str() {
            "n" => {
                position.1 += 1;
                position.2 -= 1;
            }
            "s" => {
                position.1 -= 1;
                position.2 += 1;
            }
            "ne" => {
                position.0 += 1;
                position.2 -= 1;
            }
            "sw" => {
                position.0 -= 1;
                position.2 += 1;
            }
            "se" => {
                position.0 += 1;
                position.1 -= 1;
            }
            "nw" => {
                position.0 -= 1;
                position.1 += 1;
            }
            _ => return Err(format!("Unknown direction: {}", direction)),
        }
        distance = position.distance(&Position::origin());
        if distance > max_distance {
            max_distance = distance;
        }
    }
    Ok((distance, max_distance))
}

fn main() -> Result<()> {
    let directions = parse_directions("resources/day11_input.csv")?;
    let (distance, max_distance) = solve(&directions)?;
    println!("part 1: {}", &distance);
    println!("part 2: {}", &max_distance);
    Ok(())
}
