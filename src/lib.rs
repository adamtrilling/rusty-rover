mod instruction;
mod orientation;
mod plateau;
mod rover;

use plateau::{ParsePlateauError, Plateau};

pub fn execute(input: &str) -> Result<String, ParsePlateauError> {
    let plateau = match input.parse::<Plateau>() {
        Ok(plateau) => plateau,
        Err(_) => return Err(ParsePlateauError {}),
    };

    Ok(plateau
        .rovers()
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
