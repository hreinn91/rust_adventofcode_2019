mod day1;
mod day1_tests;


pub mod utils {
    use std::fs;

    pub fn get_input(input_file: &str) -> Vec<i32> {
        let raw_input = fs::read_to_string(input_file)
            .expect("Failed reading file");

        let input: Vec<i32> = raw_input
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect();
        return input;
    }
}

fn main() {
    let part1_input = "src/input_day1.out";
    println!("Part1 needed fuel: {}", day1::calc_fuel_part1(part1_input));
}