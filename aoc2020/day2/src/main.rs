use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input() -> Vec<PasswordPolicy> {
    let mut result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        let raw_result = line
            .unwrap()
            .split(|c| c == '-' || c == ' ' || c == ':')
            .map(String::from)
            .collect::<Vec<String>>();

        result.push(PasswordPolicy {
            min: raw_result[0].parse::<u32>().unwrap(),
            max: raw_result[1].parse::<u32>().unwrap(),
            letter: raw_result[2].parse::<char>().unwrap(),
            password: raw_result[4].to_owned(),
        });
    }
    result
}

fn part1(input: &Vec<PasswordPolicy>) -> u32 {
    let mut result = 0;

    for line in input {
        let mut count = 0;

        for letter in line.password.chars() {
            if letter == line.letter {
                count += 1;
            }
        }
        if count >= line.min && count <= line.max {
            result += 1;
        }
    }
    result
}

fn part2(input: &Vec<PasswordPolicy>) -> u32 {
    let mut result = 0;

    for line in input {
        let idx1 = usize::try_from(line.min - 1).unwrap_or(0);
        let idx2 = usize::try_from(line.max - 1).unwrap_or(0);

        if (line.password.chars().nth(idx1).unwrap() == line.letter)
            ^ (line.password.chars().nth(idx2).unwrap() == line.letter)
        {
            result += 1;
        }
    }
    result
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
