use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input() -> Vec<i32> {
    let mut result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        match line {
            Ok(l) => result.push(l.parse::<i32>().unwrap_or(0)),
            Err(_) => (),
        };
    }
    result
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut diffs = HashMap::<i32, i32>::new();

    for num in input {
        match diffs.get(&(2020 - num)) {
            Some(n) => return (2020 - n) * num,
            None => diffs.insert(*num, 2020 - num),
        };
    }
    -1
}

fn part2(input: &Vec<i32>) -> i32 {
    // TODO: Fix brute force implementation
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    -1
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
