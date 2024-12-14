use aocode::day4;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day4part1(c: &mut Criterion) {
    let data = include_str!("../input/day04.txt");
    c.bench_function("day4 part1", |b| b.iter(|| day4::part1(black_box(data))));
}

pub fn day4part2(c: &mut Criterion) {
    let data = include_str!("../input/day04.txt");
    c.bench_function("day4 part2", |b| b.iter(|| day4::part2(black_box(data))));
}

criterion_group!(benches, day4part1, day4part2);
criterion_main!(benches);
