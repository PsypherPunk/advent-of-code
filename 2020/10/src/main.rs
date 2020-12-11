use std::fs;

use ::day10::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let joltages = get_adapter_joltages(&input);
    let diffs = get_joltage_differences(&joltages);

    println!(
        "What is the number of 1-jolt differences multiplied by the number of 3-jolt differences? {}",
        diffs.get(&1).unwrap() * diffs.get(&3).unwrap(),
    );

    println!(
        "What is the total number of distinct ways you can…connect the charging outlet to your device? {}",
        get_distinct_ways(&joltages),
    );
}
