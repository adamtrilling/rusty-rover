#[derive(Debug, PartialEq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn turn_left(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }

    pub fn turn_right(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::East,
            Orientation::West => Orientation::North,
            Orientation::South => Orientation::West,
            Orientation::East => Orientation::South,
        }
    }
}