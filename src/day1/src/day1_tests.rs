#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn it_works() {
        let x = day1::calc_fuel_part1("file");
        println!("{}", x);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn day1_test() {
        assert_eq!(day1::fuel_equation(12), 2);
        assert_eq!(day1::fuel_equation(14), 2);
        assert_eq!(day1::fuel_equation(1969), 654);
        assert_eq!(day1::fuel_equation(100756), 33583);
    }

    #[test]
    fn day1_part1_test() {
        let part_input_file = "src/input_day1.out";
        let needed_fuel = day1::calc_fuel_part1(part_input_file);
        println!("Part1: Needed fuel {}", needed_fuel);
    }

    #[test]
    fn day1_part2_test() {
        let part_input_file = "src/input_day1.out";
        let needed_fuel = day1::calc_fuel_part2(part_input_file);
        println!("Part2: Needed fuel {}", needed_fuel);
    }
}