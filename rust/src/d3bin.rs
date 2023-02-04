use std::fs::read_to_string;

mod d3lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d3.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d3wide.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {}", d3lib::part1(&input));
    println!("part 2: {}", d3lib::part2(&input));
}
