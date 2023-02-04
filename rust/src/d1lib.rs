pub fn part1(input: &str) -> usize {
    let mut max = 0;
    let input = input.as_bytes();
    let mut groupind = 0;
    while groupind < input.len() {
        let mut current = 0;
        while groupind < input.len() {
            let mut num = 0;
            let mut cpy = 0;
            let mut c = input[cpy + groupind];
            if c == b'\n' {
                groupind += 1;
                break;
            }
            while c != b'\n' {
                num = num * 10 + (c - b'0') as usize;
                cpy += 1;
                c = input[cpy + groupind];
            }
            current += num;
            groupind += cpy + 1;
        }
        max = max.max(current);
    }
    max
}

#[allow(dead_code)]
pub fn part1_wao(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut top = [0; 3];
    let input = input.as_bytes();
    let mut groupind = 0;
    while groupind < input.len() {
        let mut current = 0;
        while groupind < input.len() {
            let mut num = 0;
            let mut cpy = 0;
            let mut c = input[cpy + groupind];
            if c == b'\n' {
                groupind += 1;
                break;
            }
            while c != b'\n' {
                num = num * 10 + (c - b'0') as usize;
                cpy += 1;
                c = input[cpy + groupind];
            }
            current += num;
            groupind += cpy + 1;
        }
        try_insert_topn(&mut top, current);
    }
    top.into_iter().sum()
}

fn try_insert_topn<const N: usize>(list: &mut [usize; N], new: usize) {
    for i in 0..N {
        if new > list[i] {
            for j in (i..N - 1).rev() {
                list[j + 1] = list[j];
            }
            list[i] = new;
            break;
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d1test.txt").unwrap();
        assert_eq!(super::part1(&input), 24000);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d1test.txt").unwrap();
        assert_eq!(super::part2(&input), 45000);
    }
}
