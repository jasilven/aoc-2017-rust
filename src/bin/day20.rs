use failure::Error;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Clone)]
struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

fn parse_input(fname: &str) -> Result<Vec<Particle>> {
    let file = fs::File::open(fname)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let nums: std::result::Result<Vec<i64>, std::num::ParseIntError> = line?
            .chars()
            .filter(|ch| ch.is_digit(10) || ch == &'-' || ch == &',')
            .collect::<String>()
            .split(',')
            .map(|num| num.parse::<i64>())
            .collect();

        let nums = nums?;

        result.push(Particle {
            position: (nums[0], nums[1], nums[2]),
            velocity: (nums[3], nums[4], nums[5]),
            acceleration: (nums[6], nums[7], nums[8]),
        });
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

fn solve2(particles: &mut Vec<Particle>) -> usize {
    for _ in 0..1000 {
        for p in particles.iter_mut() {
            p.velocity.0 += p.acceleration.0;
            p.velocity.1 += p.acceleration.1;
            p.velocity.2 += p.acceleration.2;
            p.position.0 += p.velocity.0;
            p.position.1 += p.velocity.1;
            p.position.2 += p.velocity.2;
        }

        particles.sort_by(|a, b| a.position.cmp(&b.position));

        let mut duplicates = HashSet::new();

        for i in 1..particles.len() {
            if particles[i - 1].position == particles[i].position {
                duplicates.insert(particles[i - 1].clone());
                duplicates.insert(particles[i].clone());
            }
        }
        particles.retain(|p| !duplicates.contains(&p));
    }

    particles.len()
}

fn main() -> Result<()> {
    let mut particles = parse_input("resources/day20_input.txt")?;

    println!("part 1: {:?}", solve1(&particles));
    println!("part 2: {:?}", solve2(&mut particles));

    Ok(())
}
