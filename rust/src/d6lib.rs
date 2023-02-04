macro_rules! all_unique {
    ($($vars:ident),+$(,)?) => {
        all_unique!(@genexpr $($vars),+)
    };
    (@genexpr $first:ident, $second:ident$(, $rest:ident)+) => {
        all_unique!(@andconcat $first != $second$(, $first != $rest)+)
            && all_unique!(@genexpr $second$(, $rest)+)
    };
    (@genexpr $first:ident, $second:ident) => {
        $first != $second
    };
    (@andconcat $first:expr$(, $rest:expr)+) => {
        $first && all_unique!(@andconcat $($rest),+)
    };
    (@andconcat $last:expr) => {
        $last
    };
}

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut startind = 0;
    while startind + 4 < input.len() {
        let [a0, a1, a2, a3] = unsafe { input.as_ptr().add(startind).cast::<[u8; 4]>().read() };
        if all_unique!(a0, a1, a2, a3) {
            return startind + 4;
        }
        startind += 1;
    }
    panic!("d6p1 fail");
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut startind = 0;
    while startind + 14 < input.len() {
        let [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13] =
            unsafe { input.as_ptr().add(startind).cast::<[u8; 14]>().read() };
        if all_unique!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13) {
            return startind + 14;
        }
        startind += 1;
    }
    panic!("d6p2 fail");
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod test {
    use std::fs::read_to_string;

    const TESTS: usize = 5;

    const TESTFILES: [&str; TESTS] = [
        "../inputs/d6test1.txt",
        "../inputs/d6test2.txt",
        "../inputs/d6test3.txt",
        "../inputs/d6test4.txt",
        "../inputs/d6test5.txt",
    ];

    const P1SOLS: [usize; TESTS] = [7, 5, 6, 10, 11];
    const P2SOLS: [usize; TESTS] = [19, 23, 23, 29, 26];

    #[test]
    fn p1test() {
        for i in 0..TESTS {
            let file = TESTFILES[i];
            let solution = P1SOLS[i];
            let input = read_to_string(file).unwrap();
            assert_eq!(super::part1(&input), solution);
        }
    }

    #[test]
    fn p2test() {
        for i in 0..TESTS {
            let file = TESTFILES[i];
            let solution = P2SOLS[i];
            let input = read_to_string(file).unwrap();
            assert_eq!(super::part2(&input), solution);
        }
    }
}
