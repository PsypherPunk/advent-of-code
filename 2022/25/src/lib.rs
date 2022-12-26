use std::iter::successors;

const BALANCED_QUINARY: [char; 5] = ['0', '1', '2', '=', '-'];

pub fn get_part_one(input: &str) -> String {
    successors(
        Some(
            input
                .lines()
                .map(|line| {
                    line.bytes().fold(0, |acc, c| {
                        acc * 5
                            + match c {
                                b'=' => -2,
                                b'-' => -1,
                                _ => (c - b'0') as isize,
                            }
                    })
                })
                .sum(),
        ),
        |&n| {
            match (n as usize + 2) / 5 {
                n @ 1.. => Some(n as isize),
                _ => None,
            }
        },
    )
    .map(|n| BALANCED_QUINARY[n as usize % 5])
    .fold(String::new(), |acc, c| format!("{c}{acc}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#;

    #[test]
    fn test_part_one() {
        assert_eq!("2=-1=0".to_owned(), get_part_one(INPUT));
    }
}
