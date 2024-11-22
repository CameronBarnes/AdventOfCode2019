use derive_more::derive::IsVariant;

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, IsVariant)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_path(path: &str) -> Vec<(Direction, u16)> {
    path.split(',')
        .map(|dir| {
            let (dir, val) = dir.split_at(1);
            let val = val.parse().unwrap();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            (dir, val)
        })
        .collect()
}
