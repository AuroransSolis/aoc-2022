use std::collections::BTreeSet;

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut headpos = 0;
    let mut tailpos = 0;
    let mut positions = BTreeSet::new();
    let mut cursor = 0;
    while cursor < input.len() {
        
    }
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d9test.txt").unwrap();
        assert_eq!(super::part1(&input), 15);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d9test.txt").unwrap();
        assert_eq!(super::part2(&input), 12);
    }
}
