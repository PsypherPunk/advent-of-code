pub fn get_sum_of_identical_digits(input: &str) -> u32 {
    let mut digits = input
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

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
}
