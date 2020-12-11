use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use ::day10::*;

pub fn benchmark_get_joltage_differences(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_adapter_joltages(&input);

    c.bench_function("get_joltage_differences", |b| {
        b.iter(|| get_joltage_differences(&numbers))
    });
}

pub fn benchmark_get_distinct_ways(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_adapter_joltages(&input);

    c.bench_function("get_distinct_ways", |b| {
        b.iter(|| get_distinct_ways(&numbers))
    });
}

criterion_group!(
    benches,
    benchmark_get_joltage_differences,
    benchmark_get_distinct_ways,
);
criterion_main!(benches);
