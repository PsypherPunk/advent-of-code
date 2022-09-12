use std::collections::HashMap;

fn has_twice_and_thrice(input: &str) -> (bool, bool) {
    let mut counter = HashMap::new();

    input.chars().for_each(|c| {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    });

    let twice = counter.values().find(|&v| *v == 2);
    let thrice = counter.values().find(|&v| *v == 3);

    (twice.is_some(), thrice.is_some())
}

pub fn get_checksum(input: &str) -> usize {
    let (twos, threes): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .map(|line| has_twice_and_thrice(line))
        .unzip();

    twos.iter().filter(|&has_two| *has_two).count()
        * threes.iter().filter(|&has_three| *has_three).count()
}

fn have_one_differing_letter(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(x, y)| *x != *y).count() == 1
}

pub fn get_common_letters(input: &str) -> String {
    let candidates = input.trim().lines().collect::<Vec<_>>();

    for i in 0..candidates.len() {
        for j in i + 1..candidates.len() {
            let a = candidates[i];
            let b = candidates[j];

            if have_one_differing_letter(a, b) {
                return a
                    .chars()
                    .zip(b.chars())
                    .filter(|(x, y)| *x == *y)
                    .map(|(a, _)| a)
                    .collect::<String>();
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;

        assert_eq!(12, get_checksum(&input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;

        assert_eq!("fgij".to_owned(), get_common_letters(&input));
    }
}
