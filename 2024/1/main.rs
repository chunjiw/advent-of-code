use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn get_lists(path: &str) -> (Vec<i32>, Vec<i32>) {
    
    // Open the file
    let file_result = File::open(path);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {}", error),
    };

    // Create a buffered reader
    let reader = io::BufReader::new(file);

    let mut list1 = vec![];
    let mut list2 = vec![];

    // Iterate through lines
    for line_result in reader.lines() {
        // Unwrap the line
        let line = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem unwrapping line: {}", error),
        };
        
        // Split the line into two parts
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Convert to i32 if possible
            .collect();

        // Check if we have two numbers
        if numbers.len() == 2 {
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }
    (list1, list2)
}

fn get_distance(list1: &mut Vec<i32>, list2: &mut Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();

    let mut distance = 0;

    for i in 0..list1.len() {
        distance += (list1[i] - list2[i]).abs();
    }
    distance
}

fn get_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for number in list2 {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut similarity = 0;
    for number in list1 {
        let count = map.entry(number).or_insert(0);
        similarity += number * *count;
    }

    similarity
}

fn main() {
    
    let path = "/home/xing/projects/advent-of-code/2024/1/input.txt";
    
    let (mut list1, mut list2) = get_lists(path);
    
    let similarity = get_similarity(&list1, &list2);
    println!("The total similarity between the lists is {similarity}");
    
    let distance = get_distance(&mut list1, &mut list2);
    println!("The total distance between the lists is {distance}");
}
