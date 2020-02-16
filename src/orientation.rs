#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north_turn_left() {
        assert_eq!(Orientation::North.turn_left(), Orientation::West)
    }
    #[test]
    fn west_turn_left() {
        assert_eq!(Orientation::West.turn_left(), Orientation::South)
    }
    #[test]
    fn south_turn_left() {
        assert_eq!(Orientation::South.turn_left(), Orientation::East)
    }
    #[test]
    fn east_turn_left() {
        assert_eq!(Orientation::East.turn_left(), Orientation::North)
    }

    #[test]
    fn north_turn_right() {
        assert_eq!(Orientation::North.turn_right(), Orientation::East)
    }
    #[test]
    fn west_turn_right() {
        assert_eq!(Orientation::West.turn_right(), Orientation::North)
    }
    #[test]
    fn south_turn_right() {
        assert_eq!(Orientation::South.turn_right(), Orientation::West)
    }
    #[test]
    fn east_turn_right() {
        assert_eq!(Orientation::East.turn_right(), Orientation::South)
    }
}
