use std::fs::read_to_string;

mod d8lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d8.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d8large.txt";

#[cfg(not(feature = "large"))]
const WIDTH: usize = 99;

#[cfg(not(feature = "large"))]
const HEIGHT: usize = 99;

fn main() {
    let input = read_to_string(INPUT).unwrap();
    let processed = d8lib::read_grid::<WIDTH, HEIGHT>(&input);
    println!("part 1: {}", d8lib::part1(processed));
    println!("part 2: {}", d8lib::part2(processed));
}
