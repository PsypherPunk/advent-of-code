use std::collections::BTreeMap;

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut lines = input.trim().lines();

    let instructions = lines.next().ok_or(format!("bad input: {}", input))?;
    let nodes = lines
        .skip(1)
        .map(|line| {
            let start = &line[..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (start, (left, right))
        })
        .collect::<BTreeMap<&str, (&str, &str)>>();

    let instructions = instructions.chars().cycle();

    let steps = instructions
        .scan("AAA", |node, instruction| match *node {
            "ZZZ" => None,
            current => match nodes.get(current) {
                None => unreachable!(),
                Some((left, right)) => match instruction {
                    'L' => {
                        *node = left;
                        Some(left)
                    }
                    'R' => {
                        *node = right;
                        Some(right)
                    }
                    _ => unreachable!(),
                },
            },
        })
        .count();

    Ok(steps)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const TWO: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(2), get_part_one(ONE));
        assert_eq!(Ok(6), get_part_one(TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(ONE));
    }
}
