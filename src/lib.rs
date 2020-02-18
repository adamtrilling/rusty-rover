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

    let maybe_outputs =
        plateau
            .rovers()
            .iter()
            .try_fold("".to_string(), |res, r| match r.perform_instructions() {
                Ok(new_rover) => Ok(format!("{}\n{}", res, new_rover.to_string())),
                Err(_) => Err(ParsePlateauError {}),
            });

    match maybe_outputs {
        Ok(outputs) => Ok(outputs.trim_start().to_string()),
        Err(err) => Err(err),
    }
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
