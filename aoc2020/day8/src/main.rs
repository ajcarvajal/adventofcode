use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// struct Computer {
//     accumulator: i32,
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input() -> Vec<(String, i32)> {
    let mut result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        let op = line
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        result.push((op[0].to_owned(), op[1].parse::<i32>().unwrap()));
    }
    result
}

fn part1(input: &Vec<(String, i32)>) -> i32 {
    let mut visited_lines = vec![false; input.len()];
    let mut accumulator = 0;
    let mut i = 0_i32; // has to be i32 to add negative offsets for jmp

    while (i as usize) < input.len() {
        if visited_lines[i as usize] {
            return accumulator;
        }
        visited_lines[i as usize] = true;
        let (op, val) = &input[i as usize];
        match op.as_ref() {
            "acc" => {accumulator += val;},
            "jmp" => {i += val - 1;}, 
            "nop" | _ => {},
        }
        i += 1;
    }
    -1
}

fn part2(input: &Vec<(String, i32)>) -> i32 {
    // TODO: Fix brute force implementation
    for idx in 0..input.len() {
        let (operation, _value) = &input[idx];

        if operation == "nop" || operation == "jmp" {
            let mut visited_lines = vec![false; input.len()];
            let mut i = 0_i32; // has to be i32 to add negative offsets for jmp
            let mut runs = 0;
            let mut accumulator = 0;

            while (i as usize) < input.len() && runs < 10 {
                if visited_lines[i as usize] {
                    runs += 1;
                }
                visited_lines[i as usize] = true;
                let (op, val) = &input[i as usize];
                match op.as_ref() {
                    "acc" => {accumulator += val;},
                    "jmp" => { if i != idx as i32{ i += val - 1; } }, 
                    "nop" | _ => { if i == idx as i32 { i += val - 1 } },
                }
                i += 1;
            }
            if runs == 0 { return accumulator; }
        }
    }
    -1
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
