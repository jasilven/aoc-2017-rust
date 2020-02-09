fn solve1(
    steps: usize,
    last_val: usize,
) -> Result<(Vec<usize>, usize), Box<dyn std::error::Error>> {
    let mut buf: Vec<usize> = vec![0];
    let mut curpos = 0;

    for i in 1..=last_val {
        curpos = 1 + (curpos + steps) % buf.len();
        buf.insert(curpos, i);
    }

    Ok((buf, curpos))
}

fn solve2(steps: usize, last_val: usize) -> usize {
    let mut curpos = 0;
    let mut result = 0;

    for i in 1..=last_val {
        curpos = (curpos + steps + 1) % i;
        if curpos == 0 {
            result = i;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (buf, curpos) = solve1(355, 2017)?;

    println!("part 1: {}", buf[(curpos + 1) % buf.len()]);
    println!("part 2: {}", solve2(355, 50_000_000));

    Ok(())
}
