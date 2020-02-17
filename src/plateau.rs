use std::fmt;
use std::str::FromStr;

use super::rover::Rover;

#[derive(Debug, PartialEq)]
pub struct ParsePlateauError {}

#[derive(Debug, PartialEq)]
pub struct Plateau {
    h_size: u32,
    v_size: u32,
    rovers: Vec<Rover>,
}

impl Plateau {
    pub fn rovers(&self) -> Vec<Rover> {
        self.rovers.clone()
    }
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
