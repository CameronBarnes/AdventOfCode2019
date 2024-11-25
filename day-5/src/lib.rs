pub mod part1;
pub mod part2;

pub fn parse_program(input: &str) -> Vec<isize> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

pub fn parse_instruction(input: isize) -> (isize, bool, bool, bool) {
    let chars = input.to_string();
    if chars.len() <= 2 {
        return (input, false, false, false);
    }
    let (modes, operand) = chars.split_at(chars.len() - 2);
    let mut modes_iter = modes.chars().rev();
    (
        operand.parse().unwrap(),
        modes_iter.next().is_some_and(|c| c == '1'),
        modes_iter.next().is_some_and(|c| c == '1'),
        modes_iter.next().is_some_and(|c| c == '1'),
    )
}

pub fn run_program(program: &mut [isize], input: Vec<isize>) -> Vec<isize> {
    let mut input_iter = input.iter();
    let mut output = Vec::new();
    let mut index = 0;
    while index < program.len() {
        let operand = *program.get(index).unwrap();
        println!("Index: {index}, Operand: {operand}");
        let (operand, param1, param2, _param3) = parse_instruction(operand);

        if operand == 99 {
            break;
        }

        let pos1 = *program.get(index + 1).unwrap();
        let value1 = if param1 || operand == 3 {
            pos1
        } else {
            *program.get::<usize>(pos1.try_into().unwrap()).unwrap()
        };
        match operand {
            1 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = value1 + value2;
                index += 4;
            }
            2 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = value1 * value2;
                index += 4;
            }
            3 => {
                *program
                    .get_mut::<usize>(value1.try_into().unwrap())
                    .unwrap() = *input_iter.next().unwrap();
                index += 2;
            }
            4 => {
                println!("{value1}");
                output.push(value1);
                index += 2;
            }
            5 => {
                if value1 != 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    println!("pos2: {pos2}");
                    let value2 = if param2 {
                        pos2
                    } else {
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    index = value2.try_into().unwrap();
                }
            }
            6 => {
                if value1 == 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    let value2 = if param2 {
                        pos2
                    } else {
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    index = value2.try_into().unwrap();
                }
            }
            7 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                let store = if value1 < value2 { 1 } else { 0 };
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = store;
                index += 4;
            }
            8 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                let store = if value1 == value2 { 1 } else { 0 };
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = store;
                index += 4;
            }
            99 => {
                // We cover this case above
                unreachable!()
            }
            _ => panic!("Invalid operand: {operand}"),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../../day-2/input.txt");
        let mut program = parse_program(input);
        program[1] = 12;
        program[2] = 2;
        run_program(&mut program, Vec::new());
        let result = program.first().unwrap().to_string();
        assert_eq!("3224742", result);
    }

    #[test]
    fn simple_test() {
        let mut program = parse_program("1,0,0,0,99");
        run_program(&mut program, Vec::new());
        dbg!(&program);
        let result = program.first().unwrap().to_string();
        assert_eq!("2", result);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    #[case(-1)]
    #[case(-2)]
    #[case(-3)]
    #[case(-4)]
    fn position_eq_8(#[case] input: isize) {
        let mut program = parse_program("3,9,8,9,10,9,4,9,99,-1,8");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(input == 8, results.first().is_some_and(|val| *val == 1));
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    #[case(-1)]
    #[case(-2)]
    #[case(-3)]
    #[case(-4)]
    fn position_less_than_8(#[case] input: isize) {
        let mut program = parse_program("3,9,7,9,10,9,4,9,99,-1,8");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(input < 8, results.first().is_some_and(|val| *val == 1));
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    #[case(-1)]
    #[case(-2)]
    #[case(-3)]
    #[case(-4)]
    fn immediate_eq_8(#[case] input: isize) {
        let mut program = parse_program("3,3,1108,-1,8,3,4,3,99");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(input == 8, results.first().is_some_and(|val| *val == 1));
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    #[case(-1)]
    #[case(-2)]
    #[case(-3)]
    #[case(-4)]
    fn immediate_less_than_8(#[case] input: isize) {
        let mut program = parse_program("3,3,1107,-1,8,3,4,3,99");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(input < 8, results.first().is_some_and(|val| *val == 1));
    }

    #[rstest]
    #[case(1, 999)]
    #[case(2, 999)]
    #[case(3, 999)]
    #[case(4, 999)]
    #[case(5, 999)]
    #[case(6, 999)]
    #[case(7, 999)]
    #[case(8, 1000)]
    #[case(-1, 999)]
    #[case(-2, 999)]
    #[case(-3, 999)]
    #[case(-4, 999)]
    #[case(9, 1001)]
    #[case(10, 1001)]
    #[case(11, 1001)]
    #[case(12, 1001)]
    #[case(12312, 1001)]
    fn less_than_greater_equal(#[case] input: isize, #[case] expected: isize) {
        let mut program = parse_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(*results.first().unwrap(), expected);
    }
}
