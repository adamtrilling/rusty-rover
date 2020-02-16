use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseOrientationError {}

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

impl FromStr for Orientation {
    type Err = ParseOrientationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Orientation::North),
            "S" => Ok(Orientation::South),
            "E" => Ok(Orientation::East),
            "W" => Ok(Orientation::West),
            _ => Err(ParseOrientationError {}),
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Orientation::North => "N",
            Orientation::West => "W",
            Orientation::South => "S",
            Orientation::East => "E",
        };
        f.write_str(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        assert_eq!("N".parse::<Orientation>().unwrap(), Orientation::North);
        assert_eq!("S".parse::<Orientation>().unwrap(), Orientation::South);
        assert_eq!("E".parse::<Orientation>().unwrap(), Orientation::East);
        assert_eq!("W".parse::<Orientation>().unwrap(), Orientation::West);
        assert!("q".parse::<Orientation>().is_err());
    }

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
