use std::fs::read_to_string;

mod d7lib;

#[cfg(any(
    all(feature = "d7large", any(feature = "d7deep1", feature = "d7deep2")),
    all(feature = "d7deep1", any(feature = "d7large", feature = "d7deep2")),
    all(feature = "d7deep2", any(feature = "d7large", feature = "d7deep1"))
))]
compile_error!("Only one of `d7large`, `d7deep1`, and `d7deep2` may be enabled at a time.");

#[cfg(not(any(feature = "d7large", feature = "d7deep1", feature = "d7deep2")))]
const INPUT: &str = "../inputs/d7.txt";

// This input does `cd /` and that just fucks with things. No.
#[cfg(feature = "d7large")]
const INPUT: &str = "../inputs/d7large.txt";

// This input has issues with overflowing the stack, likely from recursing too deep.
#[cfg(feature = "d7deep1")]
const INPUT: &str = "../inputs/d7deep1.txt";

// This input has issues with overflowing the stack, likely from recursing too deep.
#[cfg(feature = "d7deep2")]
const INPUT: &str = "../inputs/d7deep2.txt";

fn main() {
    let input = read_to_string(INPUT).unwrap();
    println!("part 1: {:?}", d7lib::part1(&input));
    println!("part 2: {:?}", d7lib::part2(&input));
}
