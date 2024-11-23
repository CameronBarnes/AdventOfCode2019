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
            },
            3 => {
                *program.get_mut::<usize>(value1.try_into().unwrap()).unwrap() = *input_iter.next().unwrap();
                index += 2;
            },
            4 => {
                output.push(value1);
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
}
