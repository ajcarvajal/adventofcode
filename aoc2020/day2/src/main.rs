use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryFrom;


#[derive(Debug)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}


fn get_input() -> Vec<PasswordPolicy> {
    let mut raw_result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        raw_result.push(match line {
            Ok(l) => {
                l.split(|c| c == '-' || c == ' ' || c == ':')
                 .map(String::from)
                 .collect::<Vec<String>>()
            },
            Err(_) => {vec![]},
        });
    }

    // Parse fields into a struct
    let mut result = Vec::<PasswordPolicy>::new();
    for line in raw_result {
        result.push( 
            PasswordPolicy { min: line[0].parse::<u32>().unwrap(),
                             max: line[1].parse::<u32>().unwrap(),
                             letter: line[2].parse::<char>().unwrap(),
                             password: line[4].clone() }
            );
        
    }
    result
}


fn part1(input: &Vec<PasswordPolicy>) -> u32 {
    let mut result = 0;

    for password in input {
        let mut count = 0;

        for letter in password.password.chars() {
            if letter == password.letter {
                count += 1;
            }
        }
        if count >= password.min && count <= password.max { result += 1; }
    }
    result
}


fn part2(input: &Vec<PasswordPolicy>) -> u32 {
    let mut result = 0;

    for password in input {
        let idx1 = usize::try_from(password.min - 1).unwrap_or(0);
        let idx2 = usize::try_from(password.max - 1).unwrap_or(0);

        if (password.password.chars().nth(idx1).unwrap() == password.letter) ^ 
            (password.password.chars().nth(idx2).unwrap() == password.letter) {
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
