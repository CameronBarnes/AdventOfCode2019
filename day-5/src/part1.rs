use crate::{parse_program, run_program};


#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut program = parse_program(input);
    dbg!(&program);
    let output = run_program(&mut program, vec![1]);
    println!("Output: {output:?}");
    output.last().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("15386262", process(input));
    }
}
