use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use ::day15::*;

pub fn benchmark_get_nth_number_for_input_2020(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_numbers(&input);

    c.bench_function("get_nth_number_for_input_2020", |b| {
        b.iter(|| get_nth_number_for_input(&numbers, 2_020))
    });
}

pub fn benchmark_get_nth_number_for_input_30000000(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_numbers(&input);

    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(10);
    group.bench_function("get_nth_number_for_input_30000000", |b| {
        b.iter(|| get_nth_number_for_input(&numbers, 30_000_000))
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_get_nth_number_for_input_2020,
    benchmark_get_nth_number_for_input_30000000,
);
criterion_main!(benches);
