use std::fs::read_to_string;

mod d1lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d1.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d1large.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {}", d1lib::part1(&input));
    println!("part 2: {}", d1lib::part2(&input));
}
