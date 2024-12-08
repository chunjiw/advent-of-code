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

    let result: i128 = equations.iter().map(|eqn| calibrate(eqn, false)).sum();
    let concat_result: i128 = equations.iter().map(|eqn| calibrate(eqn, true)).sum();

    println!("The total calibration result is {result}.");
    println!("The total calibration result is {concat_result} if include concat.");
    // calibrate(&equations[0], true);
}

fn calibrate(equation: &String, concat: bool) -> i128 {
    let nums: Vec<&str> = equation.split(':').collect();
    let target: i128 = nums[0].parse().unwrap();
    let mut queue: Vec<i128> = vec![];
    for numstr in nums[1].split(' ') {
        if numstr.is_empty() { continue; }
        let num: i128 = numstr.parse().unwrap();
        if queue.is_empty() { queue.push(num); }
        else {
            let n = queue.len();
            for i in 0..n {
                queue.push(queue[i] * num);
                if concat {
                    queue.push(queue[i] * 10_i128.pow(numstr.len() as u32) + num);
                }
                queue[i] += num;

            }
        }
        // println!("{:?}", queue);
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