use std::cmp::Ordering;
use std::collections::HashSet;

fn get_memory_banks(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_whitespace()
        .map(|blocks| blocks.parse().unwrap())
        .collect()
}

fn get_max_position(memory_banks: &[usize]) -> usize {
    memory_banks
        .iter()
        .enumerate()
        .max_by(|(pos_a, val_a), (pos_b, val_b)| match val_a.cmp(&val_b) {
            Ordering::Equal => pos_b.cmp(&pos_a),
            other => other,
        })
        .unwrap()
        .0
}

fn cycle_memory_banks(memory_banks: &mut Vec<usize>) {
    let mut position = get_max_position(&memory_banks);
    let mut to_redistribute = memory_banks[position];
    memory_banks[position] = 0;

    while to_redistribute > 0 {
        position = (position + 1) % memory_banks.len();
        memory_banks[position] += 1;
        to_redistribute -= 1;
    }
}

pub fn get_steps_to_repeat(input: &str) -> usize {
    let mut seen = HashSet::new();
    let mut memory_banks = get_memory_banks(input);

    let mut steps = 0;
    while !seen.contains(&memory_banks) {
        seen.insert(memory_banks.clone());
        cycle_memory_banks(&mut memory_banks);
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "0 2 7 0";

        assert_eq!(5, get_steps_to_repeat(&input));
    }
}
