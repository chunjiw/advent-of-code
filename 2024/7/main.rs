use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    // Get the file path from the arguments
    let file_path = &args[1];

    let equations = read_as_vec(file_path);

    let results: Vec<i64> = equations.iter().map(|eqn| calibrate(eqn)).collect();
    println!("{:?}", results);

    let result: i64 = equations.iter().map(|eqn| calibrate(eqn)).sum();

    println!("The total calibration result is {result}.");
    // calibrate(&equations[0]);
}

fn calibrate(equation: &String) -> i64 {
    let nums: Vec<&str> = equation.split(':').collect();
    let target: i64 = nums[0].parse().unwrap();
    let mut queue: Vec<i64> = vec![];
    for numstr in nums[1].split(' ') {
        if numstr.is_empty() { continue; }
        let num: i64 = numstr.parse().unwrap();
        if queue.is_empty() { queue.push(num); }
        else {
            let n = queue.len();
            for i in 0..n {
                queue.push(queue[i] * num);
                queue[i] += num;
            }
        }
    }
    if queue.iter().any(|&x| x == target) { target } else { 0 }
}

fn read_as_vec(file_path: &String) -> Vec<String> {
    // Open the file
    let file_result = File::open(file_path);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {}", error),
    };

    // Create a buffered reader
    let reader = io::BufReader::new(file);

    let mut lines = vec![];
    // Iterate through lines
    for line_result in reader.lines() {
        // Unwrap the line
        let line = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem unwrapping line: {}", error),
        };
        lines.push(line);
    }
    lines
}