#[derive(Clone, Copy, Debug)]
struct Top {
    above: usize,
    label: u8,
}

impl Top {
    fn new() -> Self {
        Top { above: 0, label: 0 }
    }
}

// Assumption: labels are 4 apart.
pub fn setup<const W: usize>(input: &str) -> usize {
    let input = input.as_bytes();
    let mut stacksind = 0;
    while !input[stacksind + 1].is_ascii_digit() {
        stacksind += W * 4;
    }
    stacksind + W * 4 + 1
}

// fn find_input_stack_top<const W: usize>(input: &str, stackind: usize) -> usize {
//     let mut labelind = hoffset;
//     while input[labelind] == b' ' {
//         labelind += W * 4;
//     }
//     labelind
// }

pub fn part1<const W: usize>(input: &str, stacksind: usize) -> [u8; W] {
    let input = input.as_bytes();
    let mut stacktops = [Top::new(); W];
    let mut stacktops_ops = [(); W].map(|_| Vec::with_capacity(W));
    for (i, top) in (0..W).zip(stacktops.iter_mut()) {
        stacktops_ops[i].push(top);
    }
    let mut instrsind = input.len() - 2;
    while instrsind > stacksind {
        let toind = util::readnum_rev::<usize, b' '>(input, &mut instrsind) - 1;
        instrsind -= 4;
        let fromind = util::readnum_rev::<usize, b' '>(input, &mut instrsind) - 1;
        instrsind -= 6;
        let moveamt = util::readnum_rev::<usize, b' '>(input, &mut instrsind);
        instrsind -= 6;
        stacktops_ops[fromind]
            .iter_mut()
            .for_each(|top| top.above += moveamt);
        while let Some(mut top) = stacktops_ops[toind].pop() {
            if top.above < moveamt {
                top.above = moveamt - top.above - 1;
                stacktops_ops[fromind].push(top);
            } else {
                stacktops_ops[toind].push(top);
                break;
            }
        }
        stacktops_ops[toind]
            .iter_mut()
            .for_each(|top| top.above -= moveamt);
    }
    stacktops_ops
        .iter_mut()
        .enumerate()
        .for_each(|(stackid, tops)| {
            let hoffset = stackid * 4 + 1;
            for top in tops.iter_mut() {
                let mut labelind = hoffset;
                while input[labelind] == b' ' {
                    labelind += W * 4;
                }
                labelind += top.above * W * 4;
                top.label = input[labelind];
            }
        });
    drop(stacktops_ops);
    stacktops.map(|top| top.label)
}

pub fn part2<const W: usize>(input: &str, stacksind: usize) -> [u8; W] {
    let input = input.as_bytes();
    let mut stacktops = [Top::new(); W];
    let mut stacktops_ops = [(); W].map(|_| Vec::with_capacity(W));
    for (i, top) in (0..W).zip(stacktops.iter_mut()) {
        stacktops_ops[i].push(top);
    }
    let mut instrsind = input.len() - 2;
    while instrsind > stacksind {
        let toind = util::readnum_rev::<usize, b' '>(input, &mut instrsind) - 1;
        instrsind -= 4;
        let fromind = util::readnum_rev::<usize, b' '>(input, &mut instrsind) - 1;
        instrsind -= 6;
        let moveamt = util::readnum_rev::<usize, b' '>(input, &mut instrsind);
        instrsind -= 6;
        stacktops_ops[fromind]
            .iter_mut()
            .for_each(|top| top.above += moveamt);
        if let Some(remstart) = (0..stacktops_ops[toind].len())
            .rev()
            .take_while(|&ind| stacktops_ops[toind][ind].above < moveamt)
            .last()
        {
            let half = (stacktops_ops[toind].len() + remstart) / 2;
            let prevlen = stacktops_ops[toind].len();
            for remind in remstart..half {
                let taken = stacktops_ops[toind].swap_remove(remind);
                stacktops_ops[fromind].push(taken);
            }
            for _ in half..prevlen {
                let taken = stacktops_ops[toind].pop().unwrap();
                stacktops_ops[fromind].push(taken);
            }
        }
        stacktops_ops[toind]
            .iter_mut()
            .for_each(|top| top.above -= moveamt);
    }
    stacktops_ops
        .iter_mut()
        .enumerate()
        .for_each(|(stackid, tops)| {
            let hoffset = stackid * 4 + 1;
            for top in tops.iter_mut() {
                let mut labelind = hoffset;
                while input[labelind] == b' ' {
                    labelind += W * 4;
                }
                labelind += top.above * W * 4;
                top.label = input[labelind];
            }
        });
    drop(stacktops_ops);
    stacktops.map(|top| top.label)
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d5test.txt").unwrap();
        let stacksind = super::setup::<3>(&input);
        let p1arr = super::part1::<3>(&input, stacksind);
        let p1out = unsafe { std::str::from_utf8_unchecked(&p1arr) };
        assert_eq!(p1out, "CMZ");
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d5test.txt").unwrap();
        let stacksind = super::setup::<3>(&input);
        let p1arr = super::part2::<3>(&input, stacksind);
        let p1out = unsafe { std::str::from_utf8_unchecked(&p1arr) };
        assert_eq!(p1out, "MCD");
    }
}
