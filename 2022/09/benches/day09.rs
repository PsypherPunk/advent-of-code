use std::fs;

use day09::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    c.bench_function("part_one input", |b| {
        b.iter(|| get_part_one(black_box(&input)))
    });

    c.bench_function("part_two input", |b| {
        b.iter(|| get_part_two(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
