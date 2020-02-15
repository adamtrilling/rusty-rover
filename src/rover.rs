use super::orientation::Orientation;

#[derive(Debug, PartialEq)]
enum Instruction {
    Move,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Rover {
    x: u32,
    y: u32,
    orientation: Orientation,
    instructions: Vec<Instruction>,
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
