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

        let plateau_size: Vec<&str> = lines.next().unwrap().split(' ').collect();
        let mut rovers = Vec::new();
        while let Some(line) = lines.next() {
            let rover_str = format!("{}\n{}", line, lines.next().unwrap());
            rovers.push(rover_str.parse::<Rover>().unwrap());
        }

        Ok(Self {
            h_size: plateau_size[0].parse::<u32>().unwrap(),
            v_size: plateau_size[1].parse::<u32>().unwrap(),
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

fn execute(input: &str) -> String {
    let plateau = input.parse::<Plateau>().unwrap();
    plateau
        .rovers
        .iter()
        .map(|r| r.perform_instructions().to_string())
        .collect::<Vec<String>>()
        .join("\n")
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

        assert_eq!(execute(input), expected_output)
    }
}
