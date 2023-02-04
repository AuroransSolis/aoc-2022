use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

mod d1lib;
mod d2lib;
mod d3lib;
mod d4lib;
mod d5lib;
mod d6lib;
mod d7lib;
mod d8lib;

#[cfg(any(
    all(feature = "d7large", any(feature = "d7deep1", feature = "d7deep2")),
    all(feature = "d7deep1", any(feature = "d7large", feature = "d7deep2")),
    all(feature = "d7deep2", any(feature = "d7large", feature = "d7deep1"))
))]
compile_error!("Only one of `d7large`, `d7deep1`, and `d7deep2` may be enabled at a time.");

pub fn bench_d1(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d1.txt").unwrap();
    // #[cfg(feature = "large")]
    // let input = read_to_string("../inputs/d1large.txt").unwrap();
    c.bench_function("d1p1", |b| b.iter(|| black_box(d1lib::part1(&input))));
    c.bench_function("d1p1 wao", |b| {
        b.iter(|| black_box(d1lib::part1_wao(&input)))
    });
    c.bench_function("d1p2", |b| b.iter(|| black_box(d1lib::part2(&input))));
}

pub fn bench_d2(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d2.txt").unwrap();
    #[cfg(feature = "large")]
    let input = read_to_string("../inputs/d2large.txt").unwrap();
    c.bench_function("d2p1", |b| b.iter(|| black_box(d2lib::part1(&input))));
    c.bench_function("d2p2", |b| b.iter(|| black_box(d2lib::part2(&input))));
}

pub fn bench_d3(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d3.txt").unwrap();
    #[cfg(feature = "large")]
    let input = read_to_string("../inputs/d3wide.txt").unwrap();
    c.bench_function("d3p1", |b| b.iter(|| black_box(d3lib::part1(&input))));
    c.bench_function("d3p2", |b| b.iter(|| black_box(d3lib::part2(&input))));
}

pub fn bench_d4(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d4.txt").unwrap();
    // #[cfg(feature = "large")]
    // let input = read_to_string("../inputs/d4large.txt").unwrap();
    c.bench_function("d4p1", |b| b.iter(|| black_box(d4lib::part1(&input))));
    c.bench_function("d4p2", |b| b.iter(|| black_box(d4lib::part2(&input))));
}

pub fn bench_d5(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d5.txt").unwrap();
    #[cfg(not(feature = "large"))]
    const WIDTH: usize = 9;
    #[cfg(feature = "large")]
    let input = read_to_string("../inputs/d5large-3.txt").unwrap();
    #[cfg(feature = "large")]
    const WIDTH: usize = 5000;
    let stacksind = d5lib::setup::<WIDTH>(&input);
    c.bench_function("d5p1", |b| {
        b.iter(|| black_box(d5lib::part1::<WIDTH>(&input, stacksind)))
    });
    c.bench_function("d5p2", |b| {
        b.iter(|| black_box(d5lib::part2::<WIDTH>(&input, stacksind)))
    });
}

pub fn bench_d6(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d6.txt").unwrap();
    #[cfg(feature = "large")]
    let input = read_to_string("../inputs/d6extra.txt").unwrap();
    c.bench_function("d6p1", |b| b.iter(|| black_box(d6lib::part1(&input))));
    c.bench_function("d6p2", |b| b.iter(|| black_box(d6lib::part2(&input))));
}

pub fn bench_d7(c: &mut Criterion) {
    #[cfg(not(any(feature = "d7large", feature = "d7deep1", feature = "d7deep2")))]
    let input = read_to_string("../inputs/d7.txt").unwrap();
    #[cfg(feature = "d7large")]
    let input = read_to_string("../inputs/d7large.txt").unwrap();
    #[cfg(feature = "d7deep1")]
    let input = read_to_string("../inputs/d7deep1.txt").unwrap();
    #[cfg(feature = "d7deep2")]
    let input = read_to_string("../inputs/d7deep2.txt").unwrap();
    c.bench_function("d7p1", |b| b.iter(|| black_box(d7lib::part1(&input))));
    c.bench_function("d7p2", |b| b.iter(|| black_box(d7lib::part2(&input))));
}

pub fn bench_d8(c: &mut Criterion) {
    #[cfg(not(feature = "large"))]
    let input = read_to_string("../inputs/d8.txt").unwrap();
    #[cfg(feature = "large")]
    let input = read_to_string("../inputs/d8large-3.txt").unwrap();
    #[cfg(not(feature = "large"))]
    const WIDTH: usize = 99;
    #[cfg(feature = "large")]
    const WIDTH: usize = 0;
    #[cfg(not(feature = "large"))]
    const HEIGHT: usize = 99;
    #[cfg(feature = "large")]
    const WIDTH: usize = 0;
    let processed = d8lib::read_grid::<WIDTH, HEIGHT>(&input);
    c.bench_function("d8p1", |b| b.iter(|| black_box(d8lib::part1(processed))));
    c.bench_function("d8p2", |b| b.iter(|| black_box(d8lib::part2(processed))));
}

criterion_group! {
    // benches, bench_d3
    benches, bench_d1, bench_d2, bench_d3, bench_d4, bench_d5, bench_d6, bench_d7, bench_d8
}

criterion_main!(benches);
