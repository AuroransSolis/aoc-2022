// use rayon::prelude::*;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

pub fn read_grid<const W: usize, const H: usize>(input: &str) -> [[u8; W]; H] {
    let input = input.as_bytes();
    let mut grid = [[0; W]; H];
    for h in 0..H {
        let map_h = h * (W + 1);
        for w in 0..W {
            grid[h][w] = input[map_h + w] - b'0';
        }
    }
    grid
}

const MASK_P1: u8 = 0b00001111;
const FLAG_P1: u8 = 0b00010000;

pub fn part1<const W: usize, const H: usize>(mut input: [[u8; W]; H]) -> usize {
    let mut count = 2 * (W + H - 2);
    for rownum in 1..H - 1 {
        let mut max = input[rownum][0];
        for i in 1..W - 1 {
            if max == 9 {
                break;
            }
            let r = input[rownum][i];
            if r > max {
                count += 1;
                max = r & MASK_P1;
                input[rownum][i] |= FLAG_P1;
            }
        }
        max = input[rownum][W - 1];
        for i in (1..W - 1).rev() {
            if max == 9 {
                break;
            }
            let l = input[rownum][i];
            let lm = l & MASK_P1;
            if lm > max {
                count += 1 - (l as usize >> 4);
                max = lm;
                input[rownum][i] |= FLAG_P1;
            }
        }
    }
    for colnum in 1..W - 1 {
        let mut max = input[0][colnum];
        for i in 1..H - 1 {
            if max == 9 {
                break;
            }
            let b = input[i][colnum];
            let bm = b & MASK_P1;
            if bm > max {
                count += 1 - (b as usize >> 4);
                max = bm;
                input[i][colnum] |= FLAG_P1;
            }
        }
        max = input[H - 1][colnum];
        for i in (1..H - 1).rev() {
            if max == 9 {
                break;
            }
            let t = input[i][colnum];
            let tm = t & MASK_P1;
            if tm > max {
                count += 1 - (t as usize >> 4);
                max = tm;
            }
        }
    }
    count
}

pub fn part2<const W: usize, const H: usize>(input: [[u8; W]; H]) -> usize {
    // let mut max = 0;
    // for r in 1..H - 1 {
    //     for c in 1..W - 1 {
    //         let new = score_at(input, r, c);
    //         max = max.max(new);
    //     }
    // }
    // max
    let max = Arc::new(AtomicUsize::new(0));
    let r = Arc::new(AtomicUsize::new(1));
    std::thread::scope(|s| {
        for _ in 0..num_cpus::get() {
            s.spawn(|| {
                let mut maxtl = 0;
                loop {
                    let rtl = r.fetch_add(1, Ordering::AcqRel);
                    if rtl < H - 1 {
                        for c in 1..W - 1 {
                            let score = score_at(input, rtl, c);
                            println!("{}: {score}", rtl * (W + 1) + c);
                            maxtl = maxtl.max(score);
                        }
                    } else {
                        break;
                    }
                }
                max.fetch_max(maxtl, Ordering::AcqRel);
            });
        }
    });
    max.load(Ordering::Acquire)
}

fn score_at<const W: usize, const H: usize>(input: [[u8; W]; H], r: usize, c: usize) -> usize {
    let w = dist_w(input, r, c);
    let e = dist_e(input, r, c);
    let s = dist_s(input, r, c);
    let n = dist_n(input, r, c);
    w * e * s * n
}

fn dist_w<const W: usize, const H: usize>(input: [[u8; W]; H], r: usize, c: usize) -> usize {
    let cmp = input[r][c];
    for i in (0..c).rev() {
        if input[r][i] >= cmp {
            return c - i;
        }
    }
    c
}

fn dist_e<const W: usize, const H: usize>(input: [[u8; W]; H], r: usize, c: usize) -> usize {
    let cmp = input[r][c];
    for i in c + 1..W {
        if input[r][i] >= cmp {
            return i - c;
        }
    }
    W - c - 1
}

fn dist_s<const W: usize, const H: usize>(input: [[u8; W]; H], r: usize, c: usize) -> usize {
    let cmp = input[r][c];
    for i in r + 1..H {
        if input[i][c] >= cmp {
            return i - r;
        }
    }
    H - r - 1
}

fn dist_n<const W: usize, const H: usize>(input: [[u8; W]; H], r: usize, c: usize) -> usize {
    let cmp = input[r][c];
    for i in (0..r).rev() {
        if input[i][c] >= cmp {
            return r - i;
        }
    }
    r
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d8test.txt").unwrap();
        let processed = super::read_grid::<5, 5>(&input);
        assert_eq!(super::part1::<5, 5>(processed), 21);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d8test.txt").unwrap();
        let processed = super::read_grid::<5, 5>(&input);
        assert_eq!(super::part2::<5, 5>(processed), 8);
    }
}
