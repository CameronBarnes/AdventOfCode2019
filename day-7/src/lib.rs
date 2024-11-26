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
