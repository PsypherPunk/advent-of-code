use std::fs::File;
use std::io::prelude::*;

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn parse_digits(input: &str) -> Vec<isize> {
    input
        .lines()
        .flat_map(|line| line.chars().map(|n| n.to_digit(10).unwrap() as isize))
        .collect::<Vec<isize>>()
}

fn get_pattern(digits: &[isize], position: usize) -> Vec<isize> {
    let base = vec![0, 1, 0, -1];

    let mut pattern = base
        .iter()
        .flat_map(|digit| vec![*digit as isize; position])
        .collect::<Vec<_>>();
    let repeat = (digits.len() / pattern.len()) + 1;
    pattern = pattern
        .iter()
        .copied()
        .cycle()
        .take(pattern.len() * repeat)
        .collect::<Vec<_>>();
    Vec::from(&pattern[1..=digits.len()])
}

fn fft(digits: &[isize], phase: usize) -> String {
    let mut input = digits.to_owned();

    for _ in 0..phase {
        let mut elements: Vec<isize> = Vec::new();
        for position in 1..=input.len() {
            let pattern = get_pattern(&input, position);

            let output: isize = input.iter().zip(pattern.iter()).map(|(a, b)| *a * *b).sum();
            elements.push(output.abs() % 10);
        }
        input = elements.clone();
    }
    let output = input
        .iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>();
    output.join("")
}

fn main() {
    let input = read_input();
    let digits = parse_digits(&input);
    let output = fft(&digits, 100);
    println!(
        "…what are the first eight digits in the final output list? {}",
        &output[..8]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12345678() {
        let input = String::from("12345678");
        let digits = parse_digits(&input);
        assert_eq!("01029498", fft(&digits, 4));
    }

    #[test]
    fn test_80871224585914546619083218645595() {
        let input = String::from("80871224585914546619083218645595");
        let digits = parse_digits(&input);
        let output = fft(&digits, 100);
        assert_eq!("24176176", &output[..8]);
    }

    #[test]
    fn test_19617804207202209144916044189917() {
        let input = String::from("19617804207202209144916044189917");
        let digits = parse_digits(&input);
        let output = fft(&digits, 100);
        assert_eq!("73745418", &output[..8]);
    }

    #[test]
    fn test_69317163492948606335995924319873() {
        let input = String::from("69317163492948606335995924319873");
        let digits = parse_digits(&input);
        let output = fft(&digits, 100);
        assert_eq!("52432133", &output[..8]);
    }
}
