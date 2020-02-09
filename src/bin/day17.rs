fn solve1(steps: usize, last_val: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let mut buf: Vec<usize> = vec![0];
    let mut curpos = 0;

    for i in 1..=last_val {
        curpos = 1 + (curpos + steps) % buf.len();
        buf.insert(curpos, i);
    }

    Ok(buf[(curpos + 1) % buf.len()])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("part 1: {}", solve1(355, 2017)?);

    Ok(())
}
