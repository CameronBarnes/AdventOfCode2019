use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{parse_program, run_program};

fn calculate(program: &[isize], input: (isize, isize, isize, isize, isize)) -> isize {
    let mut output = run_program(&mut program.to_owned(), vec![input.0, 0]);
    output = run_program(
        &mut program.to_owned(),
        vec![input.1, *output.first().unwrap()],
    );
    output = run_program(
        &mut program.to_owned(),
        vec![input.2, *output.first().unwrap()],
    );
    output = run_program(
        &mut program.to_owned(),
        vec![input.3, *output.first().unwrap()],
    );
    output = run_program(
        &mut program.to_owned(),
        vec![input.4, *output.first().unwrap()],
    );
    *output.first().unwrap()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let program = parse_program(input);
    let iter: Vec<(isize, isize, isize, isize, isize)> = (0..=4)
        .flat_map(|first| {
            (0..=4)
                .filter(move |second| *second != first)
                .flat_map(move |second| {
                    (0..=4)
                        .filter(move |third| *third != first && *third != second)
                        .flat_map(move |third| {
                            (0..=4)
                                .filter(move |forth| {
                                    *forth != first && *forth != second && *forth != third
                                })
                                .flat_map(move |forth| {
                                    (0..=4)
                                        .filter(move |fifth| {
                                            *fifth != first
                                                && *fifth != second
                                                && *fifth != third
                                                && *fifth != forth
                                        })
                                        .map(move |fifth| (first, second, third, forth, fifth))
                                })
                        })
                })
        })
        .collect_vec();

    let result = iter
        .par_iter()
        .map(|input| (calculate(&program, *input), input))
        .max_by_key(|(num, _input)| *num)
        .unwrap();
    // println!("{result:?}");
    result.0.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", (4, 3, 2, 1, 0), 43210)]
    #[case("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", (0, 1, 2, 3, 4), 54321)]
    #[case("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", (1, 0, 4, 3, 2), 65210)]
    fn test_calculate(
        #[case] program: &str,
        #[case] input: (isize, isize, isize, isize, isize),
        #[case] result: isize,
    ) {
        assert_eq!(result, calculate(&parse_program(program), input));
    }

    #[rstest]
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210)]
    #[case(
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        54321
    )]
    #[case("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210)]
    fn test_optimize(#[case] program: &str, #[case] result: isize) {
        assert_eq!(result.to_string(), process(program));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("116680", process(input))
    }
}
