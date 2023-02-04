const OP_OFFSET: u8 = b'A';
const ME_OFFSET: u8 = b'X';

pub const POINTS_LUT: [[usize; 3]; 3] = {
    let mut scores = [[0; 3]; 3];
    let mut op = 0;
    while op < 3 {
        let mut me = 0;
        while me < 3 {
            let outcome = match [op, me] {
                [0, 0] | [1, 1] | [2, 2] => 3, // R v R => D, P v P => D, S v S => D
                [0, 1] | [1, 2] | [2, 0] => 6, // R v P => W, P v S => W, S v R => W
                [0, 2] | [1, 0] | [2, 1] => 0, // R v S => L, P v R => L, S v P => L
                _ => unreachable!(),
            };
            scores[op][me] = outcome + me + 1;
            me += 1;
        }
        op += 1;
    }
    scores
};

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut linenum = 0;
    while linenum < input.len() {
        unsafe {
            let op = input.get_unchecked(linenum) - OP_OFFSET;
            let me = input.get_unchecked(linenum + 2) - ME_OFFSET;
            sum += POINTS_LUT
                .get_unchecked(op as usize)
                .get_unchecked(me as usize);
        }
        linenum += 4;
    }
    sum
}

pub const OUTCOMES_LUT: [[usize; 3]; 3] = {
    let mut scores = [[0; 3]; 3];
    let mut op = 0;
    while op < 3 {
        let mut outcome = 0;
        while outcome < 3 {
            let me = match [op, outcome] {
                [0, 0] | [1, 2] | [2, 1] => 2, // L v R => S, W v P => S, D v S => S
                [0, 1] | [1, 0] | [2, 2] => 0, // D v R => R, L v P => R, W v S => R
                [0, 2] | [1, 1] | [2, 0] => 1, // W v R => P, D v P => P, L v S => P
                _ => unreachable!(),
            };
            scores[op][outcome] = POINTS_LUT[op][me];
            outcome += 1;
        }
        op += 1;
    }
    scores
};

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut linenum = 0;
    while linenum < input.len() {
        let op = unsafe { input.get_unchecked(linenum) - OP_OFFSET };
        let me = unsafe { input.get_unchecked(linenum + 2) - ME_OFFSET };
        unsafe {
            sum += OUTCOMES_LUT
                .get_unchecked(op as usize)
                .get_unchecked(me as usize);
        }
        linenum += 4;
    }
    sum
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d2test.txt").unwrap();
        assert_eq!(super::part1(&input), 15);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d2test.txt").unwrap();
        assert_eq!(super::part2(&input), 12);
    }
}
