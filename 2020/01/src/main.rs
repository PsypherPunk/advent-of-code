use std::fs;

fn get_parsed_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn get_two_numbers_which_equal(numbers: &[usize], sum: usize) -> (usize, usize) {
    for (i, a) in numbers.iter().enumerate() {
        for b in numbers[i..].iter() {
            if a + b == sum {
                return (*a, *b);
            }
        }
    }

    panic!("Hmmmm…this shouldn't happen 🤨")
}

fn get_three_numbers_which_equal(numbers: &[usize], sum: usize) -> (usize, usize, usize) {
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers[i..].iter().enumerate() {
            for c in numbers[(i + j)..].iter() {
                if a + b + c == sum {
                    return (*a, *b, *c);
                }
            }
        }
    }

    panic!("Hmmmm…this shouldn't happen 🤨")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_parsed_input(&input);

    let (a, b) = get_two_numbers_which_equal(&numbers, 2020);
    println!(
        "…what do you get if you multiply them together? {}, {}",
        a, b,
    );

    let (a, b, c) = get_three_numbers_which_equal(&numbers, 2020);
    println!(
        "…what is the product of the three entries that sum to 2020? {}",
        a * b * c,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"1721
979
366
299
675
1456"#;

        let numbers = get_parsed_input(&input);

        assert_eq!((1721, 299), get_two_numbers_which_equal(&numbers, 2020));
    }

    #[test]
    fn test_part_two() {
        let input = r#"1721
979
366
299
675
1456"#;

        let numbers = get_parsed_input(&input);

        assert_eq!(
            (979, 366, 675),
            get_three_numbers_which_equal(&numbers, 2020)
        );
    }
}
