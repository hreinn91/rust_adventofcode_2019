#[cfg(test)]
mod tests {
    use crate::{get_input, intcode};

    #[test]
    fn input_test() {
        let opcodes = get_input("src/test1.out");
        assert_eq!(opcodes, [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn parse_test() {
        let mut opcodes = get_input("src/test1.out");
        opcodes = intcode::intcode::parse(0, opcodes);
        assert_eq!(opcodes.len(), 12);
        assert_eq!(opcodes, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}