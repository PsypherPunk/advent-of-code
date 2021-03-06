fn get_digits(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

pub fn get_sum_of_identical_digits(input: &str) -> u32 {
    let mut digits = get_digits(input);

    let mut sum = 0;
    let mut previous_digit = *digits.iter().take(1).next().unwrap();
    digits.push(previous_digit);

    digits.iter().skip(1).for_each(|digit| {
        if *digit == previous_digit {
            sum += digit;
        }
        previous_digit = *digit;
    });

    sum
}

pub fn get_sum_of_halfway_digits(input: &str) -> u32 {
    let digits = get_digits(input);

    let offset = digits.len() / 2;

    digits
        .iter()
        .enumerate()
        .filter(|(index, digit)| {
            let other = digits.get((index + offset) % digits.len()).unwrap();
            *digit == other
        })
        .map(|(_, digit)| digit)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(3, get_sum_of_identical_digits("1122"));
        assert_eq!(4, get_sum_of_identical_digits("1111"));
        assert_eq!(0, get_sum_of_identical_digits("1234"));
        assert_eq!(9, get_sum_of_identical_digits("91212129"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(6, get_sum_of_halfway_digits("1212"));
        assert_eq!(0, get_sum_of_halfway_digits("1221"));
        assert_eq!(4, get_sum_of_halfway_digits("123425"));
        assert_eq!(12, get_sum_of_halfway_digits("123123"));
        assert_eq!(4, get_sum_of_halfway_digits("12131415"));
    }
}
