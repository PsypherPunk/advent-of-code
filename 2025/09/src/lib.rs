pub fn get_part_one(input: &str) -> Result<usize, String> {
    let tiles = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').ok_or("invalid line")?;
            let x = x.parse::<usize>().map_err(|e| e.to_string())?;
            let y = y.parse::<usize>().map_err(|e| e.to_string())?;

            Ok((x, y))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let max = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &(x1, y1))| {
            tiles
                .iter()
                .skip(i)
                .map(move |&(x2, y2)| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        })
        .max()
        .ok_or("invalid input")?;

    Ok(max)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(50), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
