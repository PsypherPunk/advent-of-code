const DIVISOR: usize = 20201227;

pub fn get_loop_size(public_key: usize) -> usize {
    let mut value = 1;
    for loop_size in 1.. {
        value *= 7;
        value %= DIVISOR;
        if value == public_key {
            return loop_size;
        }
    }
    panic!(r#"¯\_(ツ)_/¯"#)
}

pub fn get_encryption_key(public_key: usize, other_loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..other_loop_size {
        value *= public_key;
        value %= DIVISOR;
    }
    value
}

pub fn get_public_keys(input: &str) -> (usize, usize) {
    match input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>()[..]
    {
        [a, b] => (a, b),
        _ => panic!(r#"¯\_(ツ)_/¯"#),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(8, get_loop_size(5764801));
        assert_eq!(11, get_loop_size(17807724));

        assert_eq!(
            get_encryption_key(17807724, 8),
            get_encryption_key(5764801, 11),
        );
    }
}
