const LOWER_OFFSET: u8 = b'a';
const UPPER_OFFSET: u8 = b'A' - 26;
const LO_UO_DIFF: u8 = LOWER_OFFSET - UPPER_OFFSET;

fn charbyte_to_num(c: u8) -> u8 {
    let lcase_marker = (c >> 5) & 1;
    let offset = UPPER_OFFSET + lcase_marker * LO_UO_DIFF;
    c - offset
}

pub fn part1(input: &str) -> usize {
    let mut total_priority = 0;
    let input = input.as_bytes();
    let mut outerind = 0;
    while outerind < input.len() {
        let mut len = 0;
        let mut p1flags = 0u64;
        while input[outerind + len] != b'\n' {
            p1flags |= 1 << charbyte_to_num(input[outerind]);
            outerind += 1;
            len += 1;
        }
        for ind in outerind..outerind + len {
            let id = charbyte_to_num(input[ind]);
            if p1flags & (1 << id) != 0 {
                total_priority += id as usize + 1;
                break;
            }
        }
        outerind += len + 1;
        // while input[outerind + len] != b'\n' {
        //     len += 2;
        // }
        // let half = len / 2;
        // let mut p1flags = 0u64;
        // #[allow(clippy::needless_range_loop)]
        // for ind in outerind..outerind + half {
        //     p1flags |= 1 << charbyte_to_num(input[ind]);
        // }
        // #[allow(clippy::needless_range_loop)]
        // for ind in outerind + half..outerind + len {
        //     let id = charbyte_to_num(input[ind]);
        //     if p1flags & (1 << id) != 0 {
        //         total_priority += id as usize + 1;
        //         break;
        //     }
        // }
        // outerind += len + 1;
    }
    total_priority
}

macro_rules! uindex {
    ($array:ident[$index:expr]) => {
        unsafe { *$array.get_unchecked($index) }
    };
}

fn get_pack_flags(input: &[u8], ind: &mut usize) -> u64 {
    let mut packflags = 0;
    let mut c = uindex!(input[*ind]);
    while c != b'\n' {
        packflags |= 1 << charbyte_to_num(c);
        *ind += 1;
        c = uindex!(input[*ind]);
    }
    packflags
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut total_priority = 0;
    let mut outerind = 0;
    while outerind < input.len() {
        let mut lineind = outerind;
        let pack1 = get_pack_flags(input, &mut lineind);
        lineind += 1;
        let pack2 = get_pack_flags(input, &mut lineind);
        lineind += 1;
        let common = pack1 & pack2;
        while input[lineind] != b'\n' {
            let id = charbyte_to_num(input[lineind]);
            if (1 << id) & common != 0 {
                total_priority += id as usize + 1;
                break;
            }
            lineind += 1;
        }
        while input[lineind] != b'\n' {
            lineind += 1;
        }
        outerind = lineind + 1;
    }
    total_priority
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d3test.txt").unwrap();
        assert_eq!(super::part1(&input), 157);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d3test.txt").unwrap();
        assert_eq!(super::part2(&input), 70);
    }
}
