// struct ParseInputError {}

#[derive(Debug, PartialEq)]
struct Input {
    h_size: u32,
    v_size: u32,
    rovers: Vec<Rover>,
}

#[derive(Debug, PartialEq)]
struct Rover {
    x: u32,
    y: u32,
    orientation: Orientation,
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Move,
    Left,
    Right,
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
