use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        //  part1(lines);
        part2(lines);
    }
}

fn part1(lines: std::io::Lines<BufReader<File>>) {
    let mut result = 0;
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let operands = Regex::new(r"[0-9]{1,3}").unwrap();
    for line in lines.flatten() {
        let muls: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();

        for input in muls {
            let ops: Vec<i32> = operands
                .find_iter(input)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect();
            result += ops[0] * ops[1];
        }
    }
    println!("Result: {result}");
}

fn part2(lines: std::io::Lines<BufReader<File>>) {
    let mut result = 0;
    let mut enabled = true;
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(don't\(\))|(do\(\))").unwrap();
    let operands = Regex::new(r"[0-9]{1,3}").unwrap();
    for line in lines.flatten() {
        let muls: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();

        for input in muls {
            if input == "don't()" {
                enabled = false;
                continue;
            } else if input == "do()" {
                enabled = true;
                continue;
            }
            let ops: Vec<i32> = operands
                .find_iter(input)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect();
            if enabled && ops.len() == 2 {
                result += ops[0] * ops[1];
            }
        }
    }
    println!("Result: {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
