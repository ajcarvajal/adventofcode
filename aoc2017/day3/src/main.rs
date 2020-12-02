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


fn part1(input: &Vec<Vec<i32>>) -> i32 
}


// fn part2(input: &Vec<Vec<i32>>) {
// }


fn main() {
    let input = get_input();
    // println!("{:#?}", input);
    println!("Part1: {}", part1(&input));
    // println!("Part2: {}", part2(&input));
}
