use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}


fn get_input() -> Vec<Vec<i32>> {
    let mut result = vec![];
    let lines = read_lines("input.txt").expect("file error");

    for line in lines {
        result.push(match line {
            Ok(l) => {
                l.trim()
                 .split_whitespace()
                 .map(|numstr| numstr.parse::<i32>().unwrap_or(0))
                 .collect::<Vec<i32>>()
            },
            Err(_) => {vec![]},
        });
    }
    result
}


fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;

    for line in input {
        let mut min = std::i32::MAX;
        let mut max:i32 = 0;

        for num in line {
            if *num < min { min = *num; }
            if *num > max { max = *num; }
        }
        checksum += max - min;
    }
    checksum
}


fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;

    for line in input {
        let mut sorted = line.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        
        for i in 0..sorted.len() {
            for j in i+1..sorted.len() {
                if sorted[i] % sorted[j] == 0 {
                    result += sorted[i] / sorted[j];
                    break;
                }
            }
        }
    }
    result
}


fn main() {
    let input = get_input();
    // println!("{:#?}", input);
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
