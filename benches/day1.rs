use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn part1_benchmark(c: &mut Criterion) {
    let day1_file = fs::read_to_string("data/day1.txt").unwrap();
    c.bench_function("part1", |b| b.iter(|| aoc_2023::day1::part1(black_box(&day1_file))));
}

criterion_group!(benches, part1_benchmark);
criterion_main!(benches);