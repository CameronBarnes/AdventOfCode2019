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
        // thread::sleep(Duration::from_millis(100));
        let operand = *program.get(index).unwrap();
        // println!("Index: {index}, Operand: {operand}");
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
                // println!("dest: {dest} = {value1} + {value2}");
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
                // println!("dest: {dest} = {value1} * {value2}");
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = value1 * value2;
                index += 4;
            }
            3 => {
                let input_value = *input_iter.next().unwrap();
                // println!("dest: {value1} = {input_value}");
                *program
                    .get_mut::<usize>(value1.try_into().unwrap())
                    .unwrap() = input_value;
                index += 2;
            }
            4 => {
                // println!("output: {value1}");
                output.push(value1);
                index += 2;
            }
            5 => {
                if value1 != 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    let value2 = if param2 {
                        pos2
                    } else {
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    // println!("Index: {value2}");
                    index = value2.try_into().unwrap();
                } else {
                    index += 3;
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
                    // println!("Index = from: {pos2} = {value2}");
                    index = value2.try_into().unwrap();
                } else {
                    index += 3;
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
                // println!("dest: {dest} = {store}");
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
                // println!("dest: {dest} = {store}");
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
