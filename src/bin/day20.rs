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
        let line = line.map_err(|_| "Line parse error".to_string())?;
        let nums = line
            .chars()
            .filter(|ch| ch.is_digit(10) || ch == &'-' || ch == &',')
            .collect::<String>();

        let nums: Result<Vec<isize>, std::num::ParseIntError> =
            nums.split(',').map(|num| num.parse::<isize>()).collect();

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

fn main() -> Result<(), String> {
    let input = parse_input("resources/day20_testdata.txt")?;

    println!("{:?}", &input);
    Ok(())
}
