use std::fs::read_to_string;

mod d9lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d9.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d9large.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {}", d9lib::part1(&input));
    println!("part 2: {}", d9lib::part2(&input));
}
