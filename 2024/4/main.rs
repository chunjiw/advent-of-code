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

    // Open the file
    let file_result = File::open(file_path);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {}", error),
    };

    // Create a buffered reader
    let reader = io::BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = vec![];

    // Iterate through lines
    for line_result in reader.lines() {
        // Unwrap the line
        let line = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem unwrapping line: {}", error),
        };
        // let chars = line.chars();
        matrix.push(line.chars().collect());
    }

}
