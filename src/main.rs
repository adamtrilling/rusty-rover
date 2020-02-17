extern crate rusty_rover;

use rusty_rover::execute;

fn main() {
    let input = "5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM";
    println!("{}", execute(input).unwrap());
}
