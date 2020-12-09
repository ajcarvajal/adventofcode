use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input() -> Vec<i64> {
    let mut result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        result.push(line.unwrap().parse::<i64>().unwrap());
    }
    result
}

fn part1(input: &Vec<i64>) -> i64 {
    let mut s = HashSet::<i64>::from_iter(input[0..25].into_iter().cloned());

    for i in 26..input.len() {
        let mut found = false;
        for j in 0..i {
            if s.contains(&(input[i] - input[j])) {
                found = true;
                break;
            }
        }
        if !found {
            return input[i];
        }
        s.insert(input[i]);
    }
    0
}

fn part2(input: &Vec<i64>, target: i64) -> i64 {
    for i in 0..input.len() {
        let mut sum = input[i];
        for j in i + 1..input.len() {
            sum += input[j];
            match sum.cmp(&target) {
                Ordering::Greater => {
                    break;
                }
                Ordering::Equal => {
                    // executes once so lets forget about efficiency
                    return input[i..j].iter().min().unwrap() + input[i..j].iter().max().unwrap();
                }
                _ => {}
            }
        }
    }
    -1
}

fn main() {
    let input = get_input();
    let target = part1(&input);
    println!("Part1: {}", target);
    println!("Part2: {}", part2(&input, target));
}
