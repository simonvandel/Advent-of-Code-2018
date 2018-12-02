use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let path = "inputs/day01.txt";

    let puzzle_input = File::open(path)?;
    let buffered = BufReader::new(puzzle_input);

    let nums: Vec<isize> = buffered
        .lines()
        .map(|x| x.unwrap().parse::<isize>().unwrap())
        .collect();

    let mut part1 = 0isize;




    for num in nums.iter() {
        part1 += num;
    }

    println!("Part1: {}", part1);

    let mut acc = 0;
    let mut seen = HashSet::new();
    seen.insert(acc);

    for num in nums.into_iter().cycle() {
        acc += num;
        let already_seen = seen.replace(acc);
        if already_seen.is_some() {
            break;
        }
    }


    println!("Part2: {}", acc);

    Ok(())
}
