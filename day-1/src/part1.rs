#[tracing::instrument]
pub fn process(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap() / 3 - 2)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("3426455", process(input));
    }
}
