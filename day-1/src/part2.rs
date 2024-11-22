use rayon::iter::{ParallelBridge, ParallelIterator};

fn fuel_for_value(mut mass: usize) -> usize {
    let mut result = 0;
    while mass != 0 {
        let fuel = (mass / 3).saturating_sub(2);
        result += fuel;
        mass = fuel;
    }
    result
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .par_bridge()
        .map(|line| fuel_for_value(line.parse().unwrap()))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("5136807", process(input));
    }
}
