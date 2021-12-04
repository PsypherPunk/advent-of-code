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

fn get_gamma_rate(bits: &[isize]) -> isize {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}

fn get_epsilon_rate(bits: &[isize]) -> isize {
    bits.iter().fold(0, |result, &bit| {
        let bit = match bit {
            1 => 0,
            0 => 1,
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        };
        (result << 1) ^ bit
    })
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

    get_gamma_rate(&most_common_bits) * get_epsilon_rate(&most_common_bits)
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
        assert_eq!(198, get_part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, 2)
    }
}
