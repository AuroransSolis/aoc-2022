use std::fs::read_to_string;

mod d6lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d6.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d6extra.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {:?}", d6lib::part1(&input));
    println!("part 2: {:?}", d6lib::part2(&input));
}
