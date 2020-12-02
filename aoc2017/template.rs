use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = get_input();
    println!("Part1: {}", part1());
    // println!("Part2: {}", part2());
}

fn get_input() -> Vec<u8> {
    let mut input: Vec<u8> = Vec::new();
    let mut file = File::open("input.txt").expect("File error");
    file.read_to_end(&mut input).expect("File read error");
    input.pop(); //remove the terminating character
    
    for i in 0..input.len() {
        input[i] = input[i] - 48;
    }
    input
}

fn part1() {
}

// fn part2() {
// }
