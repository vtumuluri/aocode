use aocode::{day1, day2, day3, day4};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day1part1(c: &mut Criterion) {
    let data = include_str!("../input/day01.txt");
    c.bench_function("day1 part1", |b| b.iter(|| day1::part1(black_box(data))));
}

pub fn day1part2(c: &mut Criterion) {
    let data = include_str!("../input/day01.txt");
    c.bench_function("day1 part2", |b| b.iter(|| day1::part2(black_box(data))));
}

pub fn day2part1(c: &mut Criterion) {
    let data = include_str!("../input/day02.txt");
    c.bench_function("day2 part1", |b| b.iter(|| day2::part1(black_box(data))));
}

pub fn day2part2(c: &mut Criterion) {
    let data = include_str!("../input/day02.txt");
    c.bench_function("day2 part2", |b| b.iter(|| day2::part2(black_box(data))));
}

pub fn day3part1(c: &mut Criterion) {
    let data = include_str!("../input/day03.txt");
    c.bench_function("day3 part1", |b| b.iter(|| day3::part1(black_box(data))));
}

pub fn day3part2(c: &mut Criterion) {
    let data = include_str!("../input/day03.txt");
    c.bench_function("day3 part2", |b| b.iter(|| day3::part2(black_box(data))));
}
pub fn day4part1(c: &mut Criterion) {
    let data = include_str!("../input/day04.txt");
    c.bench_function("day4 part1", |b| b.iter(|| day4::part1(black_box(data))));
}

pub fn day4part2(c: &mut Criterion) {
    let data = include_str!("../input/day04.txt");
    c.bench_function("day4 part2", |b| b.iter(|| day4::part2(black_box(data))));
}

criterion_group!(
    benches, day1part1, day1part2, day2part1, day2part2, day3part1, day3part2, day4part1, day4part2
);
criterion_main!(benches);
