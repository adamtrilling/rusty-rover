use super::instruction::Instruction;
use super::orientation::Orientation;

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseRoverError {}

#[derive(Debug)]
pub struct OutOfBoundsError {}

#[derive(Clone, Debug, PartialEq)]
pub struct Rover {
    x: u32,
    y: u32,
    orientation: Orientation,
    instructions: Vec<Instruction>,
}

impl Rover {
    pub fn perform_instructions(&self, x_max: u32, y_max: u32) -> Result<Self, OutOfBoundsError> {
        self.instructions.iter().try_fold(self.clone(), |res, i| {
            res.perform_instruction(i, x_max, y_max)
        })
    }

    fn perform_instruction(
        &self,
        instruction: &Instruction,
        x_max: u32,
        y_max: u32,
    ) -> Result<Self, OutOfBoundsError> {
        match instruction {
            Instruction::Left => Ok(self.turn_left()),
            Instruction::Right => Ok(self.turn_right()),
            Instruction::Move => self.try_move(x_max, y_max),
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

    fn try_move(&self, x_max: u32, y_max: u32) -> Result<Self, OutOfBoundsError> {
        let (new_x, new_y) = match self.orientation {
            Orientation::North => (Some(self.x), self.y.checked_add(1)),
            Orientation::South => (Some(self.x), self.y.checked_sub(1)),
            Orientation::West => (self.x.checked_sub(1), Some(self.y)),
            Orientation::East => (self.x.checked_add(1), Some(self.y)),
        };

        if let Some(x) = new_x {
            if let Some(y) = new_y {
                if x <= x_max && y <= y_max {
                    Ok(Self {
                        x: x,
                        y: y,
                        ..self.clone()
                    })
                }
                else {
                    Err(OutOfBoundsError {})
                }
            } else {
                Err(OutOfBoundsError {})
            }
        } else {
            Err(OutOfBoundsError {})
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
        let instructions = match lines[1].chars().try_fold(Vec::new(), |mut acc, i| {
            match Instruction::try_from(i) {
                Ok(inst) => {
                    acc.push(inst);
                    Ok(acc)
                }
                Err(_) => Err(ParseRoverError {}),
            }
        }) {
            Ok(instructions) => instructions,
            Err(_) => return Err(ParseRoverError {}),
        };

        let x_pos = match position[0].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err(ParseRoverError {}),
        };

        let y_pos = match position[1].parse::<u32>() {
            Ok(y) => y,
            Err(_) => return Err(ParseRoverError {}),
        };

        let orientation = match position[2].parse::<Orientation>() {
            Ok(orientation) => orientation,
            Err(_) => return Err(ParseRoverError {}),
        };

        Ok(Rover {
            x: x_pos,
            y: y_pos,
            orientation: orientation,
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
        let new_rover = rover.try_move(20, 20).unwrap();

        assert_eq!(new_rover.y, 4)
    }

    #[test]
    fn moving_south_decreases_y() {
        let rover = Rover {
            orientation: Orientation::South,
            ..test_rover()
        };
        let new_rover = rover.try_move(20, 20).unwrap();

        assert_eq!(new_rover.y, 2)
    }

    #[test]
    fn moving_west_decreases_x() {
        let rover = Rover {
            orientation: Orientation::West,
            ..test_rover()
        };
        let new_rover = rover.try_move(20, 20).unwrap();

        assert_eq!(new_rover.x, 2)
    }

    #[test]
    fn moving_east_decreases_y() {
        let rover = Rover {
            orientation: Orientation::East,
            ..test_rover()
        };
        let new_rover = rover.try_move(20, 20).unwrap();

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

        assert_eq!(rover.perform_instructions(20, 20).unwrap(), expected_rover)
    }

    #[test]
    fn raise_error_when_moving_below_zero() {
        let rover = Rover {
            x: 0,
            orientation: Orientation::West,
            ..test_rover()
        };

        assert!(rover.perform_instruction(&Instruction::Move, 20, 20).is_err())
    }

    #[test]
    fn raise_error_when_moving_past_the_edge() {
        let rover = Rover {
            x: 5,
            ..test_rover()
        };

        assert!(rover.perform_instruction(&Instruction::Move, 5, 5).is_err())
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

    #[test]
    fn raises_error_when_parsing_invalid_x_position() {
        let rover_str = "x 2 N\nLMLMLMLMM";
        assert!(rover_str.parse::<Rover>().is_err())
    }

    #[test]
    fn raises_error_when_parsing_invalid_y_position() {
        let rover_str = "1 y N\nLMLMLMLMM";
        assert!(rover_str.parse::<Rover>().is_err())
    }

    #[test]
    fn raises_error_when_parsing_invalid_orientation() {
        let rover_str = "1 2 Z\nLMLMLMLMM";
        assert!(rover_str.parse::<Rover>().is_err())
    }

    #[test]
    fn raises_error_when_parsing_invalid_instructions() {
        let rover_str = "1 2 N\nLMLMLMLNM";
        assert!(rover_str.parse::<Rover>().is_err())
    }
}
