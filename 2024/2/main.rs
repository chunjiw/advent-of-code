use std::fs::File;
use std::io::{self, BufRead};

fn is_safe(report: &Vec<i32>) -> bool {
    report.is_sorted_by(|a, b| a < b && (b - a) <= 3) ||
    report.is_sorted_by(|a, b| a > b && (a - b) <= 3)
}

fn is_dampen_safe(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut copy = report.clone();
        copy.remove(i);
        if is_safe(&copy) {
            return true;
        }
    }
    return false;
}

fn main() {
    let path = "/home/xing/projects/advent-of-code/2024/2/input.txt";

    // Open the file
    let file_result = File::open(path);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {}", error),
    };

    // Create a buffered reader
    let reader = io::BufReader::new(file);

    let mut count = 0;
    let mut count2 = 0;

    // Iterate through lines
    for line_result in reader.lines() {
        // Unwrap the line
        let line = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem unwrapping line: {}", error),
        };
        
        // Split the line into two parts
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Convert to i32 if possible
            .collect();

        if is_safe(&report) {
            count += 1;
        }

        if is_dampen_safe(&report) {
            count2 += 1;
        }
    }
    println!("{count} reports are safe.");
    println!("{count2} reports are safe after dampening.");
}