mod instruction;
mod orientation;
mod rover;

use rover::Rover;

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ParsePlateauError {}

#[derive(Debug, PartialEq)]
struct Plateau {
    h_size: u32,
    v_size: u32,
    rovers: Vec<Rover>,
}

impl FromStr for Plateau {
    type Err = ParsePlateauError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');

        let plateau_size: Vec<&str> = match lines.next() {
            None => {
                return Err(ParsePlateauError {});
            }
            Some(sizes) => sizes.split(' ').collect(),
        };
        let mut rovers = Vec::new();
        while let Some(line) = lines.next() {
            match lines.next() {
                None => {
                    return Err(ParsePlateauError {});
                }
                Some(second_line) => {
                    let rover_str = format!("{}\n{}", line, second_line);
                    match rover_str.parse::<Rover>() {
                        Ok(rover) => rovers.push(rover),
                        Err(_) => return Err(ParsePlateauError {}),
                    }
                }
            };
        }

        let h_size = match plateau_size[0].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err(ParsePlateauError {}),
        };

        let v_size = match plateau_size[1].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err(ParsePlateauError {}),
        };

        Ok(Self {
            h_size: h_size,
            v_size: v_size,
            rovers: rovers,
        })
    }
}

impl fmt::Display for Plateau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rovers_str = self
            .rovers
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(&rovers_str)
    }
}

pub fn execute(input: &str) -> Result<String, ParsePlateauError> {
    let plateau = match input.parse::<Plateau>() {
        Ok(plateau) => plateau,
        Err(_) => return Err(ParsePlateauError {}),
    };

    Ok(plateau
        .rovers
        .iter()
        .map(|r| r.perform_instructions().to_string())
        .collect::<Vec<String>>()
        .join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM";
        let expected_output = "1 3 N
5 1 E";

        assert_eq!(execute(input).unwrap(), expected_output)
    }

    #[test]
    fn it_fails_to_parse_with_blank_input() {
        let input = "";
        assert!(input.parse::<Plateau>().is_err())
    }
}
