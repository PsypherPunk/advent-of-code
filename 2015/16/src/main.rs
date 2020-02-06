use std::fs;

static MFCSAM: &str = r#"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"#;

fn get_sues_number(sue: &str) -> u16 {
    sue
        .split(':')
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse::<u16>()
        .unwrap()
}

fn get_sue(input: &str) -> u16 {
    let mfcsam = MFCSAM.trim().lines().collect::<Vec<&str>>();

    let detected = input
        .trim()
        .lines()
        .filter(|line| {
            mfcsam
                .iter()
                .map(|detection| line.contains(detection))
                .collect::<Vec<bool>>()
                .iter()
                .filter(|&detected| *detected)
                .count()
                == 3
        })
        .collect::<Vec<&str>>();

    assert_eq!(1, detected.len());

    get_sues_number(detected.first().unwrap())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the number of the Sue that got you the gift? {}",
        get_sue(&input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
