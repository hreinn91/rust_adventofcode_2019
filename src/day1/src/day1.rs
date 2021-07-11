use crate::utils::get_input;

pub fn fuel_equation(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    return if fuel < 0 {
        0
    } else {
        fuel
    }

}

pub fn calc_fuel(mass: i32) -> i32 {
    let fuel = fuel_equation(mass);
    return if fuel <= 2 {
        fuel
    } else {
        fuel + calc_fuel(fuel)
    }
}

pub fn calc_fuel_part1(input_file: &str) -> i32 {
    let masses = get_input(input_file);
    let mut fuel = 0;
    for m in masses.iter(){
        fuel += fuel_equation(*m);
    }
    return fuel;
}

pub fn calc_fuel_part2(input_file: &str) -> i32 {
    let masses = get_input(input_file);
    let mut fuel = 0;
    masses.iter()
        .for_each(|m| fuel += calc_fuel(*m));
    return fuel;
}