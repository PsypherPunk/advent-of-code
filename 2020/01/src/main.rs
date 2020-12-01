use std::fs;

fn get_parsed_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn get_entries_which_equal(numbers: &[usize], sum: usize) -> (usize, usize) {
    for (i, a) in numbers.iter().enumerate() {
        for b in numbers[i..].iter() {
            if a + b == sum {
                return (*a, *b);
            }
        }
    }

    panic!("Hmmmm…this shouldn't happen 🤨")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_parsed_input(&input);

    let (a, b) = get_entries_which_equal(&numbers, 2020);
    println!(
        "…what do you get if you multiply them together? {}, {}",
        a, b,
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

        assert_eq!((1721, 299), get_entries_which_equal(&numbers, 2020));
    }
}
