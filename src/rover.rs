use super::instruction::Instruction;
use super::orientation::Orientation;

use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub struct ParseRoverError {}

#[derive(Clone, Debug, PartialEq)]
pub struct Rover {
    x: u32,
    y: u32,
    orientation: Orientation,
    instructions: Vec<Instruction>,
}

impl Rover {
    pub fn perform_instructions(&self) -> Self {
        self.instructions
            .iter()
            .fold(self.clone(), |res, i| res.perform_instruction(i))
    }

    fn perform_instruction(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Left => self.turn_left(),
            Instruction::Right => self.turn_right(),
            Instruction::Move => self.r#move(),
        }
    }

    fn turn_left(&self) -> Self {
        Self {
            orientation: self.orientation.turn_left(),
            ..self.clone()
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            orientation: self.orientation.turn_right(),
            ..self.clone()
        }
    }

    fn r#move(&self) -> Self {
        let (new_x, new_y) = match self.orientation {
            Orientation::North => (self.x, self.y + 1),
            Orientation::South => (self.x, self.y - 1),
            Orientation::West => (self.x - 1, self.y),
            Orientation::East => (self.x + 1, self.y),
        };

        Self {
            x: new_x,
            y: new_y,
            ..self.clone()
        }
    }
}

impl FromStr for Rover {
    type Err = ParseRoverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();

        if lines.len() != 2 {
            return Err(ParseRoverError {});
        }

        let position: Vec<&str> = lines[0].split(' ').collect();
        let instructions = lines[1]
            .chars()
            .map(|i| Instruction::try_from(i).unwrap())
            .collect();
        Ok(Rover {
            x: position[0].parse::<u32>().unwrap(),
            y: position[1].parse::<u32>().unwrap(),
            orientation: position[2].parse::<Orientation>().unwrap(),
            instructions: instructions,
        })
    }
}

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.orientation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rover() -> Rover {
        Rover {
            x: 3,
            y: 3,
            orientation: Orientation::East,
            instructions: vec![
                Instruction::Move,
                Instruction::Move,
                Instruction::Right,
                Instruction::Move,
                Instruction::Move,
                Instruction::Right,
                Instruction::Move,
                Instruction::Right,
                Instruction::Right,
                Instruction::Move,
            ],
        }
    }

    #[test]
    fn doing_turn_left_turns_left() {
        let rover = test_rover();
        let new_rover = rover.turn_left();

        assert_eq!(new_rover.orientation, Orientation::North)
    }

    #[test]
    fn doing_turn_right_turns_right() {
        let rover = test_rover();
        let new_rover = rover.turn_right();

        assert_eq!(new_rover.orientation, Orientation::South)
    }

    #[test]
    fn moving_north_increases_y() {
        let rover = Rover {
            orientation: Orientation::North,
            ..test_rover()
        };
        let new_rover = rover.r#move();

        assert_eq!(new_rover.y, 4)
    }

    #[test]
    fn moving_south_decreases_y() {
        let rover = Rover {
            orientation: Orientation::South,
            ..test_rover()
        };
        let new_rover = rover.r#move();

        assert_eq!(new_rover.y, 2)
    }

    #[test]
    fn moving_west_decreases_x() {
        let rover = Rover {
            orientation: Orientation::West,
            ..test_rover()
        };
        let new_rover = rover.r#move();

        assert_eq!(new_rover.x, 2)
    }

    #[test]
    fn moving_east_decreases_y() {
        let rover = Rover {
            orientation: Orientation::East,
            ..test_rover()
        };
        let new_rover = rover.r#move();

        assert_eq!(new_rover.x, 4)
    }

    #[test]
    fn perform_as_instructed() {
        let rover = test_rover();
        let expected_rover = Rover {
            x: 5,
            y: 1,
            orientation: Orientation::East,
            ..rover.clone()
        };

        assert_eq!(rover.perform_instructions(), expected_rover)
    }

    #[test]
    fn parses_a_valid_rover() {
        let rover_str = "1 2 N\nLMLMLMLMM";
        let expected_rover = Rover {
            x: 1,
            y: 2,
            orientation: Orientation::North,
            instructions: vec![
                Instruction::Left,
                Instruction::Move,
                Instruction::Left,
                Instruction::Move,
                Instruction::Left,
                Instruction::Move,
                Instruction::Left,
                Instruction::Move,
                Instruction::Move,
            ],
        };

        assert_eq!(rover_str.parse::<Rover>().unwrap(), expected_rover)
    }
}
