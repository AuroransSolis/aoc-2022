use std::fs::read_to_string;

mod d2lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d2.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d2large.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {}", d2lib::part1(&input));
    println!("part 2: {}", d2lib::part2(&input));
}
