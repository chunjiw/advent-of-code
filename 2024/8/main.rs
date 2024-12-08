use std::collections::HashMap;
use std::collections::HashSet;
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

    let scan = read_as_vec(file_path);

    let result = antinodes(&scan);
    let result2 = antinodes2(&scan);

    println!("There are {result} antinodes.");
    println!("There are {result2} resonant antinodes.");
}

fn antinodes2(scan: &Vec<String>) -> usize {
    let (m, n) = (scan.len(), scan[0].len());
    let mut map = HashMap::new();
    for (i, row) in scan.iter().enumerate() {
        for (j, freq) in row.chars().enumerate() {
            if freq == '.' {
                continue;
            }
            let locations = map.entry(freq).or_insert(vec![]);
            locations.push((i, j));
        }
    }
    let mut nodes = HashSet::new();
    for locations in map.values() {
        for (i, j) in locations.iter() {
            for (k, l) in locations.iter() {
                if i == k && j == l { continue; }
                nodes.insert((*i, *j));
                let mut d = 1;
                while (d+1) * k >= d*i && (d+1) * l >= d*j && (d+1) * k < m + d*i && d*l + l < d*j + n {
                    nodes.insert((d*k + k - d*i, d*l + l - d*j));
                    d += 1;
                }
            }
        }
    }
    nodes.len()
}

fn antinodes(scan: &Vec<String>) -> usize {
    let (m, n) = (scan.len(), scan[0].len());
    let mut map = HashMap::new();
    for (i, row) in scan.iter().enumerate() {
        for (j, freq) in row.chars().enumerate() {
            if freq == '.' {
                continue;
            }
            let locations = map.entry(freq).or_insert(vec![]);
            locations.push((i, j));
        }
    }
    let mut nodes = HashSet::new();
    for locations in map.values() {
        for (i, j) in locations.iter() {
            for (k, l) in locations.iter() {
                if i == k && j == l { continue; }
                if 2 * k >= *i && 2 * l >= *j && 2 * k - i < m && 2 * l - j < n {
                    nodes.insert((2 * k - i, 2 * l - j));
                }
            }
        }
    }
    nodes.len()
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