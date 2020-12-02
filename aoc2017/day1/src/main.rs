use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
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

fn part1(input: &Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..input.len() {
        if input[i] == input[i-1] {
            sum += input[i] as u32;
        }
    }
    if input[0] == input[input.len()-1] {
        sum += input[0] as u32;
    }
    sum
}

fn part2(input: &Vec<u8>) -> u32 {
    let length = input.len();

    let mut sum: u32 = 0;
    let step = length/2;
    for i in step..length {
        if input[i] == input[i-step] {
            sum += (input[i]*2) as u32;
        }
    }
    sum
}
