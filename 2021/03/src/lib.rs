use std::cmp::Ordering;

fn get_bit_counts(input: &str) -> Vec<isize> {
    let mut counts = vec![0; input.lines().next().unwrap().len()];

    input.trim().lines().for_each(|line| {
        line.trim().chars().enumerate().for_each(|(i, c)| match c {
            '1' => counts[i] += 1,
            '0' => counts[i] -= 1,
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        });
    });

    counts
}

fn get_bits_as_integer(bits: &[isize]) -> isize {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}

pub fn get_part_one(input: &str) -> isize {
    let bit_counts = get_bit_counts(input);
    let most_common_bits = bit_counts
        .iter()
        .map(|count| match count.cmp(&0) {
            Ordering::Greater | Ordering::Equal => 1,
            _ => 0,
        })
        .collect::<Vec<_>>();

    let gamma_rate = get_bits_as_integer(&most_common_bits);
    let epsilon_rate = !gamma_rate & ((1 << most_common_bits.len()) - 1);

    gamma_rate * epsilon_rate
}

fn get_most_common_bit_at(report: &Vec<&Vec<isize>>, position: usize) -> isize {
    let total = report
        .iter()
        .map(|number| match number[position] {
            1 => 1,
            0 => -1,
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        })
        .sum::<isize>();

    match total.cmp(&0) {
        Ordering::Greater | Ordering::Equal => 1,
        _ => 0,
    }
}

fn get_least_common_bit_at(report: &Vec<&Vec<isize>>, position: usize) -> isize {
    !get_most_common_bit_at(report, position) & ((1 << 1) - 1)
}

fn get_oxygen_generator_rating(report: &Vec<Vec<isize>>) -> isize {
    let mut position = 0;
    let mut most_common = get_most_common_bit_at(&report.iter().collect(), position);
    let mut input = report
        .iter()
        .filter(|number| number[position] == most_common)
        .collect::<Vec<_>>();
    if input.len() == 1 {
        return get_bits_as_integer(input[0]);
    }
    loop {
        position += 1;
        most_common = get_most_common_bit_at(&input, position);
        input = input
            .into_iter()
            .filter(|number| number[position] == most_common)
            .collect::<Vec<_>>();

        if input.len() == 1 {
            break;
        }
    }

    get_bits_as_integer(input[0])
}

fn get_co2_scrubber_rating(report: &[Vec<isize>]) -> isize {
    let mut position = 0;
    let mut least_common = get_least_common_bit_at(&report.iter().collect(), position);
    let mut input = report
        .iter()
        .filter(|number| number[position] == least_common)
        .collect::<Vec<_>>();
    if input.len() == 1 {
        return get_bits_as_integer(input[0]);
    }
    loop {
        position += 1;
        least_common = get_least_common_bit_at(&input, position);
        input = input
            .into_iter()
            .filter(|number| number[position] == least_common)
            .collect::<Vec<_>>();

        if input.len() == 1 {
            break;
        }
    }

    get_bits_as_integer(input[0])
}

pub fn get_part_two(input: &str) -> isize {
    let input = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap() as isize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    get_oxygen_generator_rating(&input) * get_co2_scrubber_rating(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(198, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(230, get_part_two(INPUT));
    }
}
