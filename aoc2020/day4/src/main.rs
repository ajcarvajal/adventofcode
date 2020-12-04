use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input() -> Vec<HashMap<String,String>> {
    let mut raw_result = vec![];

    for line in read_lines("input.txt").expect("file error") {
        raw_result.push(
            line.unwrap()
                .split(|c| c == ':' || c == ' ' || c == '\n')
                .map(String::from)
                .collect::<Vec<String>>(),
        );
    }

    let mut result = vec![];
    let mut temp = HashMap::<String,String>::new();
    
    for i in 0..raw_result.len() {
        if raw_result[i][0] == "" {
            result.push(temp.clone());
            temp.clear();
        }

        let mut j = 1;
        while j < raw_result[i].len() {
            temp.insert(raw_result[i][j-1].to_owned(),
                        raw_result[i][j].to_owned());
            j += 2;
        }

    }
    result.push(temp.clone());
    result
}

fn part1(input: &Vec<HashMap<String,String>>) -> usize {
    input.into_iter().filter(|l| !is_missing_fields(l)).count()
}

fn is_missing_fields(input: &HashMap<String,String>) -> bool {
    for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !input.contains_key(field.clone()) { 
            return true;
        }
    }
    return false
}

fn part2(input: &Vec<HashMap<String,String>>) -> u32 {
    let mut result = 0;

    let parsed_input = input.into_iter().filter(|l| !is_missing_fields(l));
    for line in parsed_input {
        match line.get("byr").unwrap().parse::<i32>() {
            Ok(n) => { if n < 1920 || n > 2002 { continue; } },
            Err(_) => continue,
        }
        match line.get("iyr").unwrap().parse::<i32>() {
            Ok(n) => { if n < 2010 || n > 2020 { continue; } },
            Err(_) => continue,
        }
        match line.get("eyr").unwrap().parse::<i32>() {
            Ok(n) => { if n < 2020 || n > 2030 { continue; } },
            Err(_) => continue,
        }

        let height = line.get("hgt").unwrap();
        if height.contains("cm") {
            let cm_height = height.split("cm").map(String::from).collect::<Vec<String>>()[0].parse::<i32>();
            match cm_height {
                Ok(n) => { if n < 150 || n > 193 { continue; } },
                Err(_) => { continue; },
            }
        } else if height.contains("in") {
            let in_height = height.split("in").map(String::from).collect::<Vec<String>>()[0].parse::<i32>();
            match in_height {
                Ok(n) => { if n < 59 || n > 76 { continue; } },
                Err(_) => { continue; },
            }
        } else {
            continue;
        }

        let hair = line.get("hcl").unwrap();
        if !hair.contains('#') || hair.len() != 7 { 
            continue;
        }

        for c in hair.split('#').collect::<String>().chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {},
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' => {},
                _ => { continue; },
            }
        }
        
        match line.get("ecl").unwrap().as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
            _ => { continue; },
        }

        let pid = line.get("pid").unwrap();
        if pid.len() != 9 { continue; } 
        for num in pid.chars() {
            match num {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {},
                _ => { continue; },
            }
        }

        result += 1;

    }
    result

}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
