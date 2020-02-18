use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Particle {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
    acceleration: (isize, isize, isize),
}

fn parse_input(fname: &str) -> Result<Vec<Particle>, String> {
    let file = fs::File::open(fname).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let nums: Result<Vec<isize>, std::num::ParseIntError> = line
            .map_err(|_| "Line parse error".to_string())?
            .chars()
            .filter(|ch| ch.is_digit(10) || ch == &'-' || ch == &',')
            .collect::<String>()
            .split(',')
            .map(|num| num.parse::<isize>())
            .collect();

        match nums {
            Ok(vec) => {
                result.push(Particle {
                    position: (vec[0], vec[1], vec[2]),
                    velocity: (vec[3], vec[4], vec[5]),
                    acceleration: (vec[6], vec[7], vec[8]),
                });
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(result)
}

fn solve1(particles: &[Particle]) -> usize {
    let mut particle = 0;
    let mut min_acceleration = usize::max_value();

    for (i, p) in particles.iter().enumerate() {
        let a = (p.acceleration.0.abs() + p.acceleration.1.abs() + p.acceleration.2.abs()).abs()
            as usize;
        if a < min_acceleration {
            min_acceleration = a;
            particle = i;
        }
    }

    particle
}

fn main() -> Result<(), String> {
    let input = parse_input("resources/day20_input.txt")?;

    println!("part 1: {:?}", solve1(&input));

    Ok(())
}
