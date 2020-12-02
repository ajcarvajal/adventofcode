use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}


fn get_input() -> Vec<Vec<String>> {
    let mut result = vec![];
    let lines = read_lines("input.txt").expect("file error");

    for line in lines {
        result.push(match line {
            Ok(l) => {
                l.trim()
                 .split_whitespace()
                 .map(String::from)
                 .collect()
            },
            Err(_) => {vec![]},
        });
    }
    result
}


fn part1(input: &Vec<Vec<String>>) -> i32 {
    let mut result = 0;

    for line in input {
        let mut wordset = HashSet::<&String>::new();
        let mut has_dupe = false;

        for word in line {
            if !wordset.insert(word) {
                has_dupe = true; break;
            }
        }
        if !has_dupe { result += 1; }
    }
    
    result
}


fn part2(input: &Vec<Vec<String>>) -> u32 {
    let mut result = 0;

    for line in input {
        let mut counts = vec![];
        let mut has_dupe = false;

        for word in line {
            let mut letter_count = HashMap::<char, u32>::new();

            for c in word.chars() {
                *letter_count.entry(c).or_insert(0) += 1;
            }

            for count in &mut counts {
                if letter_count == *count {
                    has_dupe = true;
                    break;
                }
            }
            if has_dupe { break; }
            counts.push(letter_count);
        }
        if !has_dupe { result += 1; } 
    }

    result
}

fn main() {
    let input = get_input();
    // println!("{:#?}", input);
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
