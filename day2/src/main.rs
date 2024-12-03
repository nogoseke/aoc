use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn main() {
if let Ok(lines) = read_lines("input.txt") {
//        part1(lines);
        part2(lines);
    }

}

fn part1(lines: std::io::Lines<BufReader<File>>)  {
        let mut result = 0;
        //let mut n_line = 0;
        for line in lines.flatten() {
           // println!("Line {n_line}");
           let split: Vec<i32> = line.split(' ')
                .map(|d| d.parse::<i32>().unwrap())
                .collect();
            let mut first = true;
            let mut temp = 0;
            let ascending = split[1] > split[2];
            result += 1;
            for i in split {
              if first {
                first = false;
                temp = i;
                continue;
              }
              if ascending && (i > temp){
                // println!("Ascending and failed {i}  {temp}");
                result -= 1;
                break;
            }
            if !ascending && (i < temp) {
                // println!("Descending and failed");
                result -= 1;
                break;
                
            }
              if (i - temp).abs() < 1 || (i -temp).abs() > 3 {
                // println!("Bigger step than allowed {i}  {temp}");
                result -= 1;
                break;
            }
            temp = i;
        }
        //n_line += 1;
    }
        println!("Result: {result}");
}

fn part2(lines: std::io::Lines<BufReader<File>>)  {
        let mut result = 0;
        for line in lines.flatten() {
           let split: Vec<i32> = line.split(' ')
                .map(|d| d.parse::<i32>().unwrap())
                .collect();
            let length = split.len();
            if is_valid(&split) {
                result += 1;
                continue;
            }
            for i in 0..length {
                let mut temp_report = split.clone();
                temp_report.remove(i);
                if is_valid(&temp_report) {
                    result += 1;
                    break;
                }
            }
        }
        println!("Result {result}");
}

fn is_valid(report: &Vec<i32>) -> bool {
    let mut first = true;
    let mut temp = 0;
    let ascending = report[1] > report[2];
    for i in report {
        if first {
            first = false;
            temp = *i;
            continue;
        }
        if ascending && (*i > temp){
            // println!("Ascending and failed {i}  {temp}");
            return false;
        }
        if !ascending && (*i < temp) {
            // println!("Descending and failed");
            return false;

        }
        if (*i - temp).abs() < 1 || (*i -temp).abs() > 3 {
            // println!("Bigger step than allowed {i}  {temp}");
            return false;
        }
        temp = *i;
    }
    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


