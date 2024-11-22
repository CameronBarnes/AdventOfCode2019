fn parse_program(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn run_program(program: &mut [usize]) {
    let mut index = 0;
    while index + 3 < program.len() {
        let operand = *program.get(index).unwrap();
        let pos1 = *program.get(index + 1).unwrap();
        let pos2 = *program.get(index + 2).unwrap();
        let dest = *program.get(index + 3).unwrap();
        let value1 = *program.get(pos1).unwrap();
        let value2 = *program.get(pos2).unwrap();
        match operand {
            1 => {
                *program.get_mut(dest).unwrap() = value1 + value2;
            }
            2 => {
                *program.get_mut(dest).unwrap() = value1 * value2;
            }
            99 => {
                break;
            }
            _ => panic!("Invalid operand"),
        }
        index += 4;
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut program = parse_program(input);
    program[1] = 12;
    program[2] = 2;
    run_program(&mut program);
    program.first().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let mut program = parse_program("1,0,0,0,99");
        run_program(&mut program);
        assert_eq!(2, program[0]);
    }
}
