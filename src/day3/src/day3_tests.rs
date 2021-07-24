#[cfg(test)]
mod tests {
    use crate::day3::{get_input, parse, get_grid};

    #[test]
    fn input_test() {
        let wires = get_input("src/test_input1.txt");
        assert_eq!(wires.0, ["R8", "U5", "L5", "D3"]);
        assert_eq!(wires.1, ["U7", "R6", "D4", "L4"]);
    }

    #[test]
    fn parse_test() {
        let wires = get_input("src/test_input1.txt");
        let wire_1 = wires.0;
        let wire_2 = wires.1;
        let parse1 = parse(wire_1[0].to_string());
        assert_eq!(parse1.0, "R");
        assert_eq!(parse1.1, 8);
    }

    #[test]
    fn grid_test() {
        let wires = get_input("src/test_input1.txt");
        let grid = get_grid(wires);
        assert_eq!(grid.keys().len(), 41);
    }
}