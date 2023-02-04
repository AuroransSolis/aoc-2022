use std::fs::read_to_string;

mod d4lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d4.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d4large.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {}", d4lib::part1(&input));
    println!("part 2: {}", d4lib::part2(&input));
}
