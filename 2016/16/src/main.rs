use std::fs;

fn get_checksum(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<_>>();
    let mut checksum;

    while {
        checksum = chars
            .chunks(2)
            .map(|pair| match pair[0] == pair[1] {
                true => '1',
                false => '0',
            })
            .collect::<String>();
        chars = checksum.chars().collect::<Vec<_>>();

        checksum.len() % 2 == 0
    } {}

    checksum
}

fn get_appropriately_random_data(input: &str) -> String {
    let a = input.trim().chars();
    let b = a.clone().rev().map(|ch| match ch {
        '0' => '1',
        '1' => '0',
        _ => panic!("Invalid character: {}!", ch),
    });

    [a.collect::<String>(), b.collect::<String>()].join("0")
}

fn fill_disk(state: &str, size: usize) -> String {
    let mut data = get_appropriately_random_data(state);

    while data.len() < size {
        data = get_appropriately_random_data(&data);
    }

    get_checksum(&data[..size])
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the correct checksum? {}",
        fill_disk(input.trim(), 272),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "1";

        assert_eq!("100", get_appropriately_random_data(&input));
    }

    #[test]
    fn test_0() {
        let input = "0";

        assert_eq!("001", get_appropriately_random_data(&input));
    }

    #[test]
    fn test_11111() {
        let input = "11111";

        assert_eq!("11111000000", get_appropriately_random_data(&input));
    }

    #[test]
    fn test_111100001010() {
        let input = "111100001010";

        assert_eq!(
            "1111000010100101011110000",
            get_appropriately_random_data(&input)
        );
    }

    #[test]
    fn test_110010110100_checksum() {
        let input = "110010110100";

        assert_eq!("100", get_checksum(&input));
    }

    #[test]
    fn test_state_10000_size_20() {
        assert_eq!("01100", fill_disk("10000", 20));
    }
}
