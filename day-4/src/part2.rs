use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn test_password(password: &usize) -> bool {
    let password = password.to_string().chars().collect_vec();
    password.len() == 6
        && password.windows(2).any(|window| window[0] == window[1])
        && password.windows(2).all(|window| window[0] <= window[1])
        && password
            .chunk_by(|a, b| a == b)
            .any(|chunk| chunk.len() == 2)
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (lower_bound, higher_bound) = input.trim().split_once('-').unwrap();
    let (lower_bound, higher_bound) = (
        lower_bound.parse::<usize>().unwrap(),
        higher_bound.parse::<usize>().unwrap(),
    );

    (lower_bound..=higher_bound)
        .par_bridge()
        .filter(test_password)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(132279, false)]
    #[case(133679, true)]
    #[case(111111, false)]
    #[case(223450, false)]
    #[case(123789, false)]
    fn test_password_check(#[case] input: usize, #[case] valid: bool) {
        println!("Password: {input}, should be valid: {valid}");
        assert_eq!(valid, test_password(&input));
    }
}
