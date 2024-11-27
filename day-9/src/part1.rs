use crate::{parse_program, run_program};


#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut program  = parse_program(input);
    let output = run_program(&mut program, vec![1]);
    output.last().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        todo!("Havent built test yet");
        let input = "";
        assert_eq!("", process(input));
    }
}
