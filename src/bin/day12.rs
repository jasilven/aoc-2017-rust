use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = std::result::Result<T, String>;

fn parse_numbers(ss: &[&str]) -> Result<Vec<usize>> {
    let mut result = vec![];
    for s in ss {
        match s.parse::<usize>() {
            Ok(n) => result.push(n),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(result)
}

fn parse_connections(fname: &str) -> Result<HashMap<usize, Vec<usize>>> {
    let mut result = HashMap::<_, _>::new();
    let f = File::open(fname).map_err(|e| e.to_string())?;
    for line in BufReader::new(f).lines() {
        let line = line.map_err(|e| e.to_string())?.replace("<->", ",");
        let snums: Vec<&str> = line.split(',').map(move |s| s.trim()).collect();
        let mut nums = parse_numbers(&snums)?;
        let key = nums[0];
        nums.remove(0);
        result.insert(key, nums);
    }
    Ok(result)
}

fn solve(connections: &HashMap<usize, Vec<usize>>) -> Result<Vec<HashSet<usize>>> {
    let mut visited = HashSet::<usize>::new();
    let mut result = vec![];

    for (k, nums) in connections {
        if visited.contains(k) {
            continue;
        }

        let mut group = HashSet::new();
        group.insert(*k);

        let mut childs = nums.clone();

        while !childs.is_empty() {
            let mut new_childs = vec![];

            for parent in childs {
                group.insert(parent);
                let chs = connections
                    .get(&parent)
                    .ok_or_else(|| format!("cannot find id {}", parent))?;
                for ch in chs {
                    if !group.contains(ch) {
                        new_childs.push(ch.clone())
                    }
                }
                visited.insert(parent);
            }
            childs = new_childs;
        }

        result.push(group);
        visited.insert(*k);
    }
    Ok(result)
}

fn main() -> Result<()> {
    let connections = parse_connections("resources/day12_input.txt")?;
    let groups = solve(&connections)?;
    let part1 = match groups.iter().find(|g| g.contains(&0)) {
        Some(group) => group.len(),
        None => 0,
    };
    println!("part 1: {}", part1);
    println!("part 2: {}", groups.len());

    Ok(())
}
