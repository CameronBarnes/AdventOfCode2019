fn run_attempt(mut program: Vec<usize>, input1: usize, input2: usize) -> usize {
    program[1] = input1;
    program[2] = input2;
    crate::run_program(&mut program);
    *program.first().unwrap()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let program = crate::parse_program(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_attempt(program.clone(), noun, verb) == 19690720 {
                return (100 * noun + verb).to_string()
            }
        }
    }
    "failed".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("7960", process(input));
    }
}
