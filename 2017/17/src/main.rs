use std::fs;

use ::day17::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut spinlock = Spinlock::from_str(&input.trim(), 2017);
    println!(
        "What is the value after 2017…? {}",
        spinlock.get_value_after_last_inserted(),
    );

    let mut spinlock = Spinlock::from_str(&input.trim(), 50_000_000);
    println!(
        "What is the value after 0…? {}",
        spinlock.get_value_after_zero(),
    );
}
