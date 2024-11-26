use itertools::Itertools;

fn solve(input: &str, size: usize) -> usize {
    let layer = input
        .replace('\n', "")
        .chars()
        .chunks(size)
        .into_iter()
        .map(|chunk| chunk.collect_vec())
        .sorted_by_key(|chunk| chunk.iter().filter(|pixel| **pixel == '0').count())
        .next()
        .unwrap();
    layer.iter().filter(|pixel| **pixel == '1').count()
        * layer.iter().filter(|pixel| **pixel == '2').count()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    solve(input, 25 * 6).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "123456789012";
        let size = 3 * 2;
        assert_eq!(1, solve(input, size));
    }
}
