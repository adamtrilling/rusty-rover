mod rover;

use rover::Rover;

// struct ParseInputError {}

#[derive(Debug, PartialEq)]
struct Input {
    h_size: u32,
    v_size: u32,
    rovers: Vec<Rover>,
}

fn execute(input: &str) -> String {
    "".to_string()
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
