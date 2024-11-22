use ahash::{HashSet, HashSetExt};

use crate::Direction;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (path_a, path_b) = input.split_once("\n").expect("should be exactly two lines");
    let (path_a, path_b) = (crate::parse_path(path_a.trim()), crate::parse_path(path_b.trim()));
    let mut line_a = HashSet::new();
    // let line_b = HashSet::new();
    let mut x = 0isize;
    let mut y = 0isize;

    for (dir, val) in &path_a {
        let val = *val as isize;
        for _ in 0..val {
            match dir {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x += 1,
                Direction::Right => x -= 1,
            }
            line_a.insert((x, y));
        }
    }
    x = 0;
    y = 0;
    let mut intersections = Vec::new();
    for (dir, val) in &path_b {
        let val = *val as isize;
        for _ in 0..val {
            match dir {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x += 1,
                Direction::Right => x -= 1,
            }
            if line_a.contains(&(x, y)) {
                intersections.push((x, y));
            }
        }
    }

    intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("R8,U5,L5,D3
U7,R6,D4,L4", "6")]
    #[case("R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83", "159")]
    #[case("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", "135")]
    fn test_process(#[case] input: &str, #[case] result: &str) {
        assert_eq!(result, process(input));
    }
}
