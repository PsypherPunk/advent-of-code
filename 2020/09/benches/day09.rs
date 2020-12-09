use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use ::day09::*;

pub fn benchmark_get_first_invalid_number(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_numbers(&input);

    c.bench_function("get_first_invalid_number", |b| {
        b.iter(|| get_first_invalid_number(&numbers, 25))
    });
}

pub fn benchmark_get_encryption_weakness(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_numbers(&input);
    let invalid_number = get_first_invalid_number(&numbers, 25);

    c.bench_function("get_encryption_weakness", |b| {
        b.iter(|| get_encryption_weakness(&numbers, invalid_number))
    });
}

criterion_group!(
    benches,
    benchmark_get_first_invalid_number,
    benchmark_get_encryption_weakness,
);
criterion_main!(benches);
