use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input() -> Vec<HashSet<char>> {
    let mut result = vec![];

    let mut tempset = HashSet::<char>::new();
    for line in read_lines("input.txt").expect("file error") {
        let l = line.unwrap();
        if l== "" {
            result.push(tempset.clone());
            tempset.clear();
        } else {
            for c in l.chars() {
                tempset.insert(c);
            }

        }
    }
    result.push(tempset.clone());
    result
}

fn part1(input: &Vec<HashSet<char>>) -> usize {
    let mut result = 0;

    for line in input {
        result += line.len();
    }
    result
}

struct Entry {
    letter_counts: HashMap<char, u32>,
    participants: u32,
}

impl Entry {
    fn clear(&mut self) {
        self.letter_counts.clear();
        self.participants = 0;
    }

    fn find_consensus(&self) -> u32 {
        let mut result = 0;
        for (_letter, count) in &self.letter_counts {
            if count == &self.participants {
                result += 1;
            }
        }
        result
    }
}

fn part2() -> u32 {
    let mut result = 0;

    let mut entry = Entry { letter_counts: HashMap::<char, u32>::new(),
                            participants: 0 };

    for line in read_lines("input.txt").expect("file error") {
        let l = line.unwrap();

        if l == "" {
            result += entry.find_consensus();
            entry.clear();
        } else {
            entry.participants += 1;
            for c in l.chars() {
                *entry.letter_counts.entry(c).or_insert(0) += 1;
            }

        }
    }
    result + entry.find_consensus()
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2());
}
