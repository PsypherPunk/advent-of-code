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
        .trim()
        .chars()
        .map(|n| n.to_digit(10).unwrap() as isize)
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
    input
        .iter()
        .map(|digit| digit.to_string())
        .collect::<String>()
}

fn fft2(input: &str, phases: usize) -> String {
    let offset = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut digits = input
        .chars()
        .map(|c| c.to_string().parse::<isize>().unwrap())
        .cycle()
        .take(input.len() * 10_000)
        .skip(offset)
        .collect::<Vec<isize>>();

    for _ in 0..phases {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    digits[..8]
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
}

fn main() {
    let input = read_input();
    let digits = parse_digits(&input);
    let output = fft(&digits, 100);
    println!(
        "…what are the first eight digits in the final output list? {}",
        &output[..8]
    );

    let input = read_input();
    let output = fft2(&input.trim(), 100);
    println!(
        "…what is the eight-digit message embedded in the final output list? {}",
        output,
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

    #[test]
    fn test_03036732577212944063491565474664() {
        let input = String::from("03036732577212944063491565474664");
        let output = fft2(&input, 100);
        assert_eq!("84462026", output);
    }

    #[test]
    fn test_02935109699940807407585447034323() {
        let input = String::from("02935109699940807407585447034323");
        let output = fft2(&input, 100);
        assert_eq!("78725270", output);
    }

    #[test]
    fn test_03081770884921959731165446850517() {
        let input = String::from("03081770884921959731165446850517");
        let output = fft2(&input, 100);
        assert_eq!("53553731", output);
    }
}
