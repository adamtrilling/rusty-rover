use std::convert::TryFrom;

#[derive(Debug)]
pub struct ParseInstructionError {}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Move,
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ParseInstructionError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            'M' => Ok(Instruction::Move),
            _ => Err(ParseInstructionError {}),
        }
    }
}
