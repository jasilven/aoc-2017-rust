use std::fs;

#[derive(Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_moves(path: &str) -> Result<Vec<Move>, String> {
    let data = fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .map(|s| s.trim().to_string())?;

    let mut result = vec![];

    for split in data.split(',').collect::<Vec<&str>>() {
        match split.chars().next() {
            Some('s') => {
                result.push(Move::Spin(split[1..].parse::<usize>().unwrap()));
            }
            Some('x') => {
                let splits: Vec<&str> = split[1..].split('/').collect();
                result.push(Move::Exchange(
                    splits[0].parse::<usize>().unwrap(),
                    splits[1].parse::<usize>().unwrap(),
                ));
            }
            Some('p') => {
                let splits: Vec<&str> = split[1..].split('/').collect();
                result.push(Move::Partner(
                    splits[0].chars().next().unwrap(),
                    splits[1].chars().next().unwrap(),
                ));
            }
            Some(x) => return Err(format!("Unexpected char: {}", x)),
            None => return Err("Parse error".to_string()),
        }
    }

    Ok(result)
}

fn solve1(moves: &[Move], programs: &mut [char]) {
    for m in moves.iter() {
        match m {
            Move::Spin(i) => {
                programs.rotate_right(*i);
            }
            Move::Exchange(i, j) => {
                programs.swap(*i, *j);
            }
            Move::Partner(ch1, ch2) => {
                for ch in programs.iter_mut() {
                    if ch == ch1 {
                        *ch = *ch2
                    } else if ch == ch2 {
                        *ch = *ch1
                    }
                }
            }
        }
    }
}

fn solve2(moves: &[Move], programs: &mut [char], cnt: usize) {
    let mut original: Vec<char> = programs.iter().copied().collect();

    for i in 0..cnt {
        solve1(moves, programs);
        if original == programs {
            for _ in 0..cnt % (i + 1) {
                solve1(moves, &mut original);
            }
            break;
        }
    }
    programs.clone_from_slice(&original);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let moves = parse_moves("resources/day16_input.txt")?;

    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();

    solve1(&moves, &mut programs);

    println!("part 1: {}", programs.iter().collect::<String>());

    solve2(&moves, &mut programs, 1_000_000_000 - 1);
    println!("part 2: {}", programs.iter().collect::<String>());

    Ok(())
}
