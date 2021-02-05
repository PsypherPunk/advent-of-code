mod knot;

fn get_grid(key: &str) -> Vec<String> {
    (0..128)
        .map(|row| {
            let mut knot_hash = knot::KnotHash::new(0, 255);
            let input = format!("{}-{}", key, row);

            knot_hash.hash(&input)
        })
        .collect()
}

pub fn get_used_square_count(key: &str) -> usize {
    get_grid(&key)
        .iter()
        .map(|row| {
            let binary = row
                .chars()
                .map(|c| to_binary(c))
                .collect::<Vec<_>>()
                .join("");

            binary.chars().filter(|c| *c == '1').count()
        })
        .sum()
}

fn to_binary<'a>(c: char) -> &'a str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => panic!(r#"¯\_(ツ)_/¯"#),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(8108, get_used_square_count("flqrgnkx"));
    }
}
