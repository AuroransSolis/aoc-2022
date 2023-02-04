use std::fs::read_to_string;

mod d5lib;

#[cfg(not(feature = "large"))]
const INPUT: &str = "../inputs/d5.txt";
#[cfg(feature = "large")]
const INPUT: &str = "../inputs/d5large3.txt";

#[cfg(not(feature = "large"))]
const WIDTH: usize = 9;
#[cfg(feature = "large")]
const WIDTH: usize = 5000;

fn main() {
    let input = read_to_string(INPUT).unwrap();
    let stacksind = d5lib::setup::<WIDTH>(&input);
    let p1arr = d5lib::part1::<WIDTH>(&input, stacksind);
    let p1str = unsafe { std::str::from_utf8_unchecked(&p1arr) };
    println!("part 1: '{p1str}'");
    let p2arr = d5lib::part2::<WIDTH>(&input, stacksind);
    let p2str = unsafe { std::str::from_utf8_unchecked(&p2arr) };
    println!("part 2: '{p2str}'");
}
