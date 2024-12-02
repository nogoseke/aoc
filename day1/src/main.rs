use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;


fn main() {
if let Ok(lines) = read_lines("input.txt") {
//        part1(lines);
        part2(lines);
    }

}

fn part1(lines: std::io::Lines<BufReader<File>>)  {
        let mut line_a: Vec<i32> = Vec::new();
        let mut line_b: Vec<i32> = Vec::new();
        let mut result = 0;
        for line in lines.flatten() {
           let split: Vec<i32> = line.split("   ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect();
            line_a.push(split[0]);
            line_b.push(split[1]);
        }
        line_a.sort();
        line_b.sort();
        for (index, item) in line_a.iter().enumerate(){
            result += (item-line_b[index]).abs();
        }
        println!("Result: {result}");
}

fn part2(lines: std::io::Lines<BufReader<File>>)  {
        let mut line_a: Vec<i32> = Vec::new();
        let mut line_b: Vec<i32> = Vec::new();
        let mut result = 0;
        for line in lines.flatten() {
           let split: Vec<i32> = line.split("   ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect();
            line_a.push(split[0]);
            line_b.push(split[1]);
        }
        let counts = line_b.iter().counts();

        
        for item in line_a {
            result += item * (*counts.get(&item).unwrap_or(&0) as i32);
        }
        println!("Result: {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


