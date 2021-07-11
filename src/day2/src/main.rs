mod intcode;
mod intcode_test;

use std::fs;

pub fn get_input(file: &str) -> Vec<i32> {
    let raw_input = fs::read_to_string(file)
        .expect("Failed reading file");
    let input: Vec<i32> = raw_input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    return input;
}


fn main() {
    let part1_out = intcode::intcode::run_intcode_program_part1("src/input1.out");
    println!("Part1 out: {}", part1_out);
    let part1_out = intcode::intcode::run_intcode_program_part2("src/input1.out");
    println!("Part2 out: {}", part1_out);
}
