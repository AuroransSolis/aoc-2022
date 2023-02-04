pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut count = 0;
    let mut outerind = 0;
    while outerind < input.len() {
        let mut innerind = outerind;
        let a1 = util::readnum_12::<u8, b'-'>(input, &mut innerind);
        innerind += 1;
        let a2 = util::readnum_12::<u8, b','>(input, &mut innerind);
        innerind += 1;
        let b1 = util::readnum_12::<u8, b'-'>(input, &mut innerind);
        innerind += 1;
        let b2 = util::readnum_12::<u8, b'\n'>(input, &mut innerind);
        if (a1 <= b1 && b2 <= a2) || (b1 <= a1 && a2 <= b2) {
            count += 1;
        }
        outerind = innerind + 1;
    }
    count
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut count = 0;
    let mut outerind = 0;
    while outerind < input.len() {
        let mut innerind = outerind;
        let a1 = util::readnum_12::<u8, b'-'>(input, &mut innerind);
        innerind += 1;
        let a2 = util::readnum_12::<u8, b','>(input, &mut innerind);
        innerind += 1;
        let b1 = util::readnum_12::<u8, b'-'>(input, &mut innerind);
        innerind += 1;
        let b2 = util::readnum_12::<u8, b'\n'>(input, &mut innerind);
        if (a1 <= b1 && b1 <= a2)
            || (a1 <= b2 && b2 <= a2)
            || (b1 <= a1 && a1 <= b2)
            || (b1 <= a2 && a2 <= b2)
        {
            count += 1;
        }
        outerind = innerind + 1;
    }
    count
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d4test.txt").unwrap();
        assert_eq!(super::part1(&input), 2);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d4test.txt").unwrap();
        assert_eq!(super::part2(&input), 4);
    }
}
