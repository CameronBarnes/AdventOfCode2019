

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut program = crate::parse_program(input);
    program[1] = 12;
    program[2] = 2;
    crate::run_program(&mut program);
    program.first().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() {
        let mut program = crate::parse_program("1,0,0,0,99");
        crate::run_program(&mut program);
        assert_eq!(2, program[0]);
    }
}
