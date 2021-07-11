pub mod intcode {
    use crate::get_input;

    pub fn run_intcode_program_part1(file: &str) -> i32 {
        let intcode = get_input(file);
        run_program(12, 2, intcode)
    }

    pub fn run_intcode_program_part2(file: &str) -> i32 {
        let intcode = get_input(file);
        find_noun_verb_prod(19690720, intcode)
    }

    pub fn find_noun_verb_prod(fin_state: i32, intcode: Vec<i32>) -> i32 {
        for noun in 0..100 {
            for verb in 0..100 {
                if run_program(noun, verb, intcode.to_owned()) == fin_state {
                    return 100 * noun + verb;
                }
            }
        }
        return -1;
    }

    pub fn run_program(noun: i32, verb: i32, mut intcode: Vec<i32>) -> i32 {
        intcode[1] = noun;
        intcode[2] = verb;
        let halt_state = parse(0, intcode);
        return halt_state[0];
    }

    pub fn parse(counter: usize, mut intcode: Vec<i32>) -> Vec<i32> {
        let optcode = intcode[counter];
        return if optcode == 99 {
            intcode
        } else if optcode == 1 || optcode == 2 {
            let pos1 = intcode[counter + 1];
            let value1 = intcode[pos1 as usize];
            let pos2 = intcode[counter + 2];
            let value2 = intcode[pos2 as usize];
            let position = intcode[counter + 3];
            if optcode == 1 {
                let sum = value1 + value2;
                intcode[position as usize] = sum;
            } else if optcode == 2 {
                let prod = value1 * value2;
                intcode[position as usize] = prod;
            }
            parse(counter + 4, intcode)
        } else {
            vec![-1]
        };
    }
}