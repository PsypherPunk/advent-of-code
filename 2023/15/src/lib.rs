fn hash(string: &str) -> usize {
    string
        .as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let sum = input.trim().split(',').map(hash).sum();

    Ok(sum)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(1320), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
