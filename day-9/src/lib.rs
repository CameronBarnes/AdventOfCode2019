pub mod part1;
pub mod part2;

pub fn parse_program(input: &str) -> Vec<isize> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

pub fn parse_instruction(input: isize) -> (isize, u32, u32, u32) {
    let chars = input.to_string();
    if chars.len() <= 2 {
        return (input, 0, 0, 0);
    }
    let (modes, operand) = chars.split_at(chars.len() - 2);
    let mut modes_iter = modes.chars().rev();
    (
        operand.parse().unwrap(),
        modes_iter.next().map_or(0, |c| c.to_digit(10).unwrap()),
        modes_iter.next().map_or(0, |c| c.to_digit(10).unwrap()),
        modes_iter.next().map_or(0, |c| c.to_digit(10).unwrap()),
    )
}

pub fn run_program(program: &mut Vec<isize>, input: Vec<isize>) -> Vec<isize> {
    let mut input_iter = input.iter();
    let mut output = Vec::new();
    let mut index = 0;
    let mut relative = 0isize;
    while index < program.len() {
        // thread::sleep(Duration::from_millis(100));
        let operand = *program.get(index).unwrap();
        println!("Index: {index}, Operand: {operand}");
        let (operand, param1, param2, param3) = parse_instruction(operand);

        if operand == 99 {
            break;
        }

        let pos1 = *program.get(index + 1).unwrap();
        let value1 = if param1 == 1 || operand == 3 {
            pos1
        } else if param1 == 2 {
            if (pos1 + relative) as usize >= program.len() {
                program
                    .try_reserve((pos1 + relative + 1) as usize - program.len())
                    .unwrap_or_else(|_| {
                        panic!("failed with size: {}", pos1 + relative + 1);
                    });
                program.resize_with((pos1 + relative + 1) as usize, || 0);
            }
            *program
                .get::<usize>((pos1 + relative).try_into().unwrap())
                .unwrap()
        } else {
            if pos1 as usize >= program.len() {
                program
                    .try_reserve((pos1 + 1) as usize - program.len())
                    .unwrap();
                program.resize_with((pos1 + 1) as usize, || 0);
            }
            *program.get::<usize>(pos1.try_into().unwrap()).unwrap()
        };
        match operand {
            1 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 == 1 {
                    pos2
                } else if param2 == 2 {
                    if (pos2 + relative) as usize >= program.len() {
                        program
                            .try_reserve((pos2 + relative + 1) as usize - program.len())
                            .unwrap_or_else(|_| {
                                panic!("failed with size: {}", pos2 + relative + 1);
                            });
                        program.resize_with((pos2 + relative + 1) as usize, || 0);
                    }
                    *program
                        .get::<usize>((pos2 + relative).try_into().unwrap())
                        .unwrap()
                } else {
                    if pos2 as usize >= program.len() {
                        program
                            .try_reserve((pos2 + 1) as usize - program.len())
                            .unwrap();
                        program.resize_with((pos2 + 1) as usize, || 0);
                    }
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                // FIXME: This needs to support relative mode, so we'll handle that as a special
                // case here, and then fix this nightmare code later
                let pos3 = *program.get(index + 3).unwrap();
                let dest = if param3 == 2 {
                    println!("pos3: {pos3}, relative: {relative}, result: {}", pos3 + relative);
                    pos3 + relative
                } else {
                    pos3
                };
                println!("dest: {dest} = {value1} + {value2}");
                if dest as usize >= program.len() {
                    program
                        .try_reserve((dest + 1) as usize - program.len())
                        .unwrap();
                    program.resize_with((dest + 1) as usize, || 0);
                }
                let len = program.len();
                *program
                    .get_mut::<usize>(dest.try_into().unwrap())
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get: {}, with program memory length: {}",
                            dest, len
                        );
                    }) = value1 + value2;
                index += 4;
            }
            2 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 == 1 {
                    pos2
                } else if param2 == 2 {
                    if (pos2 + relative) as usize >= program.len() {
                        program
                            .try_reserve((pos2 + relative + 1) as usize - program.len())
                            .unwrap_or_else(|_| {
                                panic!("failed with size: {}", pos2 + relative + 1);
                            });
                        program.resize_with((pos2 + relative + 1) as usize, || 0);
                    }
                    *program
                        .get::<usize>((pos2 + relative).try_into().unwrap())
                        .unwrap()
                } else {
                    if pos2 as usize >= program.len() {
                        program
                            .try_reserve((pos2 + 1) as usize - program.len())
                            .unwrap();
                        program.resize_with((pos2 + 1) as usize, || 0);
                    }
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                // FIXME: This needs to support relative mode, so we'll handle that as a special
                // case here, and then fix this nightmare code later
                let pos3 = *program.get(index + 3).unwrap();
                let dest = if param3 == 2 {
                    println!("pos3: {pos3}, relative: {relative}, result: {}", pos3 + relative);
                    pos3 + relative
                } else {
                    pos3
                };
                println!("dest: {dest} = {value1} * {value2}");
                if dest as usize >= program.len() {
                    program
                        .try_reserve((dest + 1) as usize - program.len())
                        .unwrap();
                    program.resize_with((dest + 1) as usize, || 0);
                }
                let len = program.len();
                *program
                    .get_mut::<usize>(dest.try_into().unwrap())
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get: {}, with program memory length: {}",
                            dest, len
                        );
                    }) = value1 * value2;
                index += 4;
            }
            3 => {
                let input_value = *input_iter.next().unwrap();
                // Handle the special case for this opcode
                let value1 = if param1 == 2 { pos1 + relative } else { value1 };
                println!("dest: {value1} = {input_value}");
                *program
                    .get_mut::<usize>(value1.try_into().unwrap())
                    .unwrap() = input_value;
                index += 2;
            }
            4 => {
                println!("output: {value1}");
                output.push(value1);
                index += 2;
            }
            5 => {
                if value1 != 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    let value2 = if param2 == 1 {
                        pos2
                    } else if param2 == 2 {
                        if (pos2 + relative) as usize >= program.len() {
                            program
                                .try_reserve((pos2 + relative + 1) as usize - program.len())
                                .unwrap_or_else(|_| {
                                    panic!("failed with size: {}", pos2 + relative + 1);
                                });
                            program.resize_with((pos2 + relative + 1) as usize, || 0);
                        }
                        *program
                            .get::<usize>((pos2 + relative).try_into().unwrap())
                            .unwrap()
                    } else {
                        if pos2 as usize >= program.len() {
                            program
                                .try_reserve((pos2 + 1) as usize - program.len())
                                .unwrap();
                            program.resize_with((pos2 + 1) as usize, || 0);
                        }
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    println!("Index = from: {pos2} = {value2}");
                    index = value2.try_into().unwrap();
                } else {
                    index += 3;
                }
            }
            6 => {
                if value1 == 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    let value2 = if param2 == 1 {
                        pos2
                    } else if param2 == 2 {
                        if (pos2 + relative) as usize >= program.len() {
                            program
                                .try_reserve((pos2 + relative + 1) as usize - program.len())
                                .unwrap_or_else(|_| {
                                    panic!("failed with size: {}", pos2 + relative + 1);
                                });
                            program.resize_with((pos2 + relative + 1) as usize, || 0);
                        }
                        *program
                            .get::<usize>((pos2 + relative).try_into().unwrap())
                            .unwrap()
                    } else {
                        if pos2 as usize >= program.len() {
                            program
                                .try_reserve((pos2 + 1) as usize - program.len())
                                .unwrap();
                            program.resize_with((pos2 + 1) as usize, || 0);
                        }
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    println!("Index = from: {pos2} = {value2}");
                    index = value2.try_into().unwrap();
                } else {
                    index += 3;
                }
            }
            7 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 == 1 {
                    pos2
                } else if param2 == 2 {
                    if (pos2 + relative) as usize >= program.len() {
                        program
                            .try_reserve((pos2 + relative + 1) as usize - program.len())
                            .unwrap_or_else(|_| {
                                panic!("failed with size: {}", pos2 + relative + 1);
                            });
                        program.resize_with((pos2 + relative + 1) as usize, || 0);
                    }
                    *program
                        .get::<usize>((pos2 + relative).try_into().unwrap())
                        .unwrap()
                } else {
                    if pos2 as usize >= program.len() {
                        program
                            .try_reserve((pos2 + 1) as usize - program.len())
                            .unwrap();
                        program.resize_with((pos2 + 1) as usize, || 0);
                    }
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                // FIXME: This needs to support relative mode, so we'll handle that as a special
                // case here, and then fix this nightmare code later
                let pos3 = *program.get(index + 3).unwrap();
                let dest = if param3 == 2 {
                    println!("pos3: {pos3}, relative: {relative}, result: {}", pos3 + relative);
                    pos3 + relative
                } else {
                    pos3
                };
                let store = if value1 < value2 { 1 } else { 0 };
                println!("dest: {pos3}:{dest} = {store}. {pos1}:{value1} < {pos2}:{value2}");
                if dest as usize >= program.len() {
                    program
                        .try_reserve((dest + 1) as usize - program.len())
                        .unwrap();
                    program.resize_with((dest + 1) as usize, || 0);
                }
                let len = program.len();
                *program
                    .get_mut::<usize>(dest.try_into().unwrap())
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get: {}, with program memory length: {}",
                            dest, len
                        );
                    }) = store;
                index += 4;
            }
            8 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 == 1 {
                    pos2
                } else if param2 == 2 {
                    if (pos2 + relative) as usize >= program.len() {
                        program
                            .try_reserve((pos2 + relative + 1) as usize - program.len())
                            .unwrap_or_else(|_| {
                                panic!("failed with size: {}", pos2 + relative + 1);
                            });
                        program.resize_with((pos2 + relative + 1) as usize, || 0);
                    }
                    *program
                        .get::<usize>((pos2 + relative).try_into().unwrap())
                        .unwrap()
                } else {
                    if pos2 as usize >= program.len() {
                        program
                            .try_reserve((pos2 + 1) as usize - program.len())
                            .unwrap();
                        program.resize_with((pos2 + 1) as usize, || 0);
                    }
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                // FIXME: This needs to support relative mode, so we'll handle that as a special
                // case here, and then fix this nightmare code later
                let pos3 = *program.get(index + 3).unwrap();
                let dest = if param3 == 2 {
                    println!("pos3: {pos3}, relative: {relative}, result: {}", pos3 + relative);
                    pos3 + relative
                } else {
                    pos3
                };
                let store = if value1 == value2 { 1 } else { 0 };
                println!("dest: {dest} = {store}");
                if dest as usize >= program.len() {
                    program
                        .try_reserve((dest + 1) as usize - program.len())
                        .unwrap();
                    program.resize_with((dest + 1) as usize, || 0);
                }
                let len = program.len();
                *program
                    .get_mut::<usize>(dest.try_into().unwrap())
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get: {}, with program memory length: {}",
                            dest, len
                        );
                    }) = store;
                index += 4;
            }
            9 => {
                relative += value1;
                println!("increased relative index by: {value1} to {relative}");
                index += 2;
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
    use itertools::Itertools;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_self_copy() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = parse_program(input);
        let result = run_program(&mut program, Vec::new());
        assert_eq!(input, result.into_iter().map(|c| c.to_string()).join(","));
    }

    #[test]
    fn test_example_2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = parse_program(input);
        let result = run_program(&mut program, Vec::new());
        assert_eq!(1219070632396864, *result.last().unwrap());
    }

    #[test]
    fn test_example_3() {
        let input = "104,1125899906842624,99";
        let mut program = parse_program(input);
        let result = run_program(&mut program, Vec::new());
        assert_eq!(1125899906842624, *result.last().unwrap());
    }

    #[test]
    fn test_day_2() {
        let input = include_str!("../../day-2/input.txt");
        let mut program = parse_program(input);
        program[1] = 12;
        program[2] = 2;
        run_program(&mut program, Vec::new());
        let result = program.first().unwrap().to_string();
        assert_eq!("3224742", result);
    }

    #[test]
    fn test_day_5_part1() {
        let input = include_str!("../../day-5/input.txt");
        let mut program = parse_program(input);
        let output = run_program(&mut program, vec![1]);
        println!("Output: {output:?}");
        let result = output.last().unwrap().to_string();
        assert_eq!("15386262", result);
    }

    #[test]
    fn test_day_5_part2() {
        let input = include_str!("../../day-5/input.txt");
        let mut program = parse_program(input);
        let output = run_program(&mut program, vec![5]);
        println!("Output: {output:?}");
        let result = output.last().unwrap().to_string();
        assert_eq!("10376124", result);
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

    #[rstest]
    #[case(-2)]
    #[case(-1)]
    #[case(0)]
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
    #[case(-2311)]
    #[case(12311)]
    #[case(11)]
    fn zero_if_zero_position(#[case] input: isize) {
        let mut program = parse_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(*results.first().unwrap() == 0, input == 0);
    }

    #[rstest]
    #[case(-2)]
    #[case(-1)]
    #[case(0)]
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
    #[case(-2311)]
    #[case(12311)]
    #[case(11)]
    fn zero_if_zero_immediate(#[case] input: isize) {
        let mut program = parse_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let results = run_program(&mut program, vec![input]);
        assert_eq!(*results.first().unwrap() == 0, input == 0);
    }
}
