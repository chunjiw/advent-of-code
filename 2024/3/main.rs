use regex::Regex;
use std::fs;

fn main() {
    let path = "/home/xing/projects/advent-of-code/2024/3/input.txt";

    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    let re_do = Regex::new(r"do()").unwrap();
    let re_dont = Regex::new(r"don't()").unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut prod = 0;

    let mut do_indices = vec![];
    let mut dont_indices = vec![];
    let mut mul_indices = vec![];

    for mat in re_do.find_iter(&input) {
        let i = mat.start();
        do_indices.push((i, -1));
    }

    for mat in re_dont.find_iter(&input) {
        let i = mat.start();
        dont_indices.push((i, -2));
    }
    
    for cap in re.captures_iter(&input) {
        let full_match = cap.get(0).unwrap();
        let i = full_match.start();
        // Extract the captured groups
        let num1 = cap.get(1).unwrap().as_str();
        let num2 = cap.get(2).unwrap().as_str();

        // Parse the numbers
        let n1 = num1.parse::<i32>().unwrap_or(0);
        let n2 = num2.parse::<i32>().unwrap_or(0);

        prod += n1 * n2;

        mul_indices.push((i, n1 * n2));
    }
    println!("Sum of all mul operations is {prod}.");
}