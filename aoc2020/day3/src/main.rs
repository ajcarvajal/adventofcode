use std::convert::TryFrom;
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

fn get_input() -> Vec<Vec<char>> {
    let mut result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        result.push(line.unwrap().chars().collect::<Vec<char>>());
    }
    result
}

fn part1(input: &Vec<Vec<char>>, right_steps: usize, down_steps: usize) -> u32 {
    let mut result = 0;
    let width = input[0].len();

    let mut i = 0;
    let mut j = 0;
    while i < input.len() {
        if j >= width {
            j = j - width;
        }

        if input[i][j] == '#' {
            result += 1;
        }
        i += down_steps;
        j += right_steps;
    }
    result
}

fn part2(input: &Vec<Vec<char>>) -> u32 {
    return part1(&input, 1, 1)
        * part1(&input, 3, 1)
        * part1(&input, 5, 1)
        * part1(&input, 7, 1)
        * part1(&input, 1, 2);
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input, 3, 1));
    println!("Part2: {}", part2(&input));
}
