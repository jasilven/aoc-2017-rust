use std::fs;

type Result<T> = std::result::Result<T, String>;

fn parse_input(fname: &str) -> Result<Vec<char>> {
    let s = fs::read_to_string(fname).map_err(|e| e.to_string())?;
    Ok(s.chars().collect())
}

fn eat_garbage(input: &[char], index: usize) -> (usize, usize) {
    let mut index = index + 1;
    let mut carbage = 0;
    loop {
        match input[index] {
            '>' => break,
            '!' => index += 2,
            _ => {
                index += 1;
                carbage += 1;
            }
        };
    }
    (index, carbage)
}

fn solve(input: &[char]) -> Result<(usize, usize)> {
    let mut index = 0;
    let mut level = 0;
    let mut result = 0;
    let mut carbage = 0;

    while index < input.len() {
        let ch = input[index];
        match ch {
            '<' => {
                let (i, carb) = eat_garbage(input, index);
                index = i;
                carbage += carb;
            }
            '{' => level += 1,
            '}' => {
                result += level;
                level -= 1;
            }
            _ => {}
        }
        index += 1;
    }

    Ok((result, carbage))
}

fn main() -> Result<()> {
    let input = parse_input("resources/day9_input.txt")?;
    let (part1, part2) = solve(&input)?;
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
    Ok(())
}
