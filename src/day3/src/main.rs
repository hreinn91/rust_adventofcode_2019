mod day3_tests;

pub mod day3 {
    use std::fs;
    use std::collections::HashMap;
    use std::borrow::Borrow;

    pub fn get_input(filename: &str) -> (Vec<String>, Vec<String>) {
        let raw_input = fs::read_to_string(filename)
            .expect("Failed reading file");
        let raw_instructions: Vec<&str> = raw_input
            .split("\n")
            .map(|s| s)
            .collect();

        let wire_1: Vec<String> = raw_instructions[0]
            .split(",")
            .map(|s| s.to_string())
            .collect();

        let wire_2: Vec<String> = raw_instructions[1]
            .split(",")
            .map(|s| s.to_string())
            .collect();

        (wire_1, wire_2)
    }

    pub fn print_wire_instruction(wires: (Vec<String>, Vec<String>)) {
        println!("{:?}", wires.0);
        println!("{:?}", wires.1);
    }

    // gird value 0:o, 1: -, 2: | 3: X
    pub fn get_grid(wires: (Vec<String>, Vec<String>)) -> HashMap<(i32, i32), i32> {
        let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
        populate_grid(&mut grid, wires.0);
        populate_grid(&mut grid, wires.1);

        grid
    }

    pub fn populate_grid(grid: &mut HashMap<(i32, i32), i32>, wire: Vec<String>) {
        let mut x = 0;
        let mut y = 0;
        grid.insert((x, y), 0);
        for instruction in wire {
            let new_coord = insert_coordinate(grid, parse(instruction), x, y);
            x = i32::clone(&new_coord.0);
            y = i32::clone(&new_coord.1);
        }
    }

    pub fn insert_coordinate(grid: &mut HashMap<(i32, i32), i32>,
                             instructions: (String, i32),
                             mut x: i32, mut y: i32) -> (i32, i32) {
        let dir = instructions.0;
        let len = instructions.1;
        for _i in 1..=len {
            if dir == "R" {
                x = x + 1;
            } else if dir == "L" {
                x = x - 1;
            } else if dir == "U" {
                y = y + 1;
            } else if dir == "D" {
                y = y - 1;
            }

            if grid.contains_key(&(x, y)) {
                grid.entry((x, y)).or_insert(3);
            } else if dir == "L" || dir == "R" {
                grid.insert((x, y), 1);
            } else if dir == "U" || dir == "D" {
                grid.insert((x, y), 1);
            }
        }

        (x, y)
    }

    pub fn parse(instruction: String) -> (String, i32) {
        let dir = &instruction[0..1];
        let len = &instruction[1..2];
        (dir.to_string(), len.parse().unwrap())
    }

    pub fn print_grid(grid: HashMap<(i32, i32), i32>) {
        let grid_size = get_grid_size(grid);
        for y in 0..=grid_size.1 {
            let mut row: Vec<String> = Vec::new();
            for x in 0..=grid_size.0 {
                let mut rep = if grid.get(&(x, y)).unwrap() == &1 {
                    "-"
                } else if grid.get(&(x, y)).unwrap() == &2 {
                    "|"
                } else if grid.get(&(x, y)).unwrap() == &3 {
                    "X"
                } else {
                    "."
                }.to_string();

                row.insert(x as usize, String::from(rep));
            }
            println!("{:?}", row);
        }
    }

    pub fn get_grid_size(grid: HashMap<(i32, i32), i32>) -> (i32, i32) {
        let keys = grid.keys();
        let mut x_size = 0;
        let mut y_size = 0;
        for key in keys {
            if x_size < key.0 {
                x_size = key.0;
            }
            if y_size < key.1 {
                y_size = key.1;
            }
        }
        (x_size, y_size)
    }
}

use crate::day3::{get_input, print_wire_instruction, get_grid};


fn main() {
    let wires = get_input("src/test_input1.txt");
    print_wire_instruction(wires.clone());
    get_grid(wires.clone());
}
