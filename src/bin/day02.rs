use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = "inputs/day02.txt";

    let puzzle_input = File::open(path)?;
    let buffered = BufReader::new(puzzle_input);

    let ids: Vec<String> = buffered.lines().flat_map(|x| x.ok()).collect();

    let mut two_letters = 0isize;
    let mut three_letters = 0isize;

    let mut letter_freq = HashMap::new();

    for id in ids.iter() {
        for c in id.chars() {
            letter_freq.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        if letter_freq.values().find(|x| *x == &2).is_some() {
            two_letters += 1;
        }

        if letter_freq.values().find(|x| *x == &3).is_some() {
            three_letters += 1;
        }

        letter_freq.clear();
    }

    let checksum = two_letters * three_letters;

    println!("Part1: {}", checksum);

    // ----- part 2------------
    let part2 = part2(ids);
    println!("Part2: {}", part2);

    Ok(())
}

fn part2(ids: Vec<String>) -> String {
    for id1 in ids.iter() {
        for id2 in ids.iter() {
            let diffs: Vec<isize> = id1
                .chars()
                .zip(id2.chars())
                .map(|(x, y)| isize::abs(x as isize - y as isize))
                .collect();

            if diffs.iter().filter(|x| *x == &0).count() == diffs.len() - 1 {
                let _1diff = diffs
                    .iter()
                    .enumerate()
                    .find(|(_, x)| *x != &0)
                    .map(|x| x.0);

                let mut ret = id2.clone();
                let idx = _1diff.unwrap();
                ret.remove(idx);
                return ret;
            }
        }
    }
    unreachable!("string must be found")
}
