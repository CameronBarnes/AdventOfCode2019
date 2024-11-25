use crate::{parse_program, run_program};


#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut program = parse_program(input);
    let output = run_program(&mut program, vec![5]);
    println!("Output: {output:?}");
    output.last().unwrap().to_string()
}

// TODO: See the TODO in the lib.rs file

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        return;
        todo!("Havent built test yet");
        let input = "";
        assert_eq!("", process(input));
    }
}
