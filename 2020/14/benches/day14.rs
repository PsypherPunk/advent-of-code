use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use ::day14::*;

pub fn benchmark_read_initialization_program(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    c.bench_function("read_initialization_program", |b| {
        b.iter(|| read_initialization_program(&input))
    });
}

pub fn benchmark_read_initialization_program_v2(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    c.bench_function("read_initialization_program_v2", |b| {
        b.iter(|| read_initialization_program_v2(&input))
    });
}

criterion_group!(
    benches,
    benchmark_read_initialization_program,
    benchmark_read_initialization_program_v2,
);
criterion_main!(benches);
