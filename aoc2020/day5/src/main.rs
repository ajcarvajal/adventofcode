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

fn get_input() -> Vec<String> {
    let mut result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        result.push(line.unwrap());
    }
    result
}

fn run(input: &Vec<String>) {
    let mut max = 0.0f32;
    let mut seats = [[' '; 8]; 128];

    for line in input {
        let mut left_row = 0.0f32;
        let mut right_row: f32 = 127.0f32;
        let mut left_col: f32 = 0.0f32;
        let mut right_col: f32 = 7.0f32;
        for c in line.chars() {
            let row_range = ((right_row - left_row) / 2.0).ceil();
            let col_range = ((right_col - left_col) / 2.0).ceil();
            match c {
                'F' => { right_row -= row_range },
                'B' => { left_row += row_range },
                'L' => { right_col -= col_range },
                'R' => { left_col += col_range },
                _ => {},
            }
        }
        seats[left_row as usize][left_col as usize] = 'x';
        if max < left_row * 8.0 + left_col { 
            max = left_row * 8.0 + left_col;
        }
    }
    println!("Part 1: {}", max);

    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            if seats[row][col] == ' ' {
                if row > 10 && row < 110 {
                    println!("Part 2: Empty seat at {},{}", row, col);
                }
            }
        }
    }
}

fn main() {
    let input = get_input();
    run(&input);
}
