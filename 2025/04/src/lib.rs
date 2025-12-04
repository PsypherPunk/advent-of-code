pub fn get_part_one(input: &str) -> Result<usize, String> {
    let width = input.find('\n').ok_or("invalid input")?;

    let bytes = input.as_bytes();

    let bytes_per_row = width + 1;
    let height = bytes.len() / bytes_per_row;

    let mut rolls = 0;

    for y in 0..height {
        let row_start = y * bytes_per_row;
        for x in 0..width {
            let idx = row_start + x;
            if bytes[idx] != b'@' {
                continue;
            }

            let mut adjacent = 0;
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
                        continue;
                    }
                    let i = (ny as usize) * bytes_per_row + (nx as usize);
                    if bytes[i] == b'@' {
                        adjacent += 1;
                    }
                }
            }

            if adjacent < 4 {
                rolls += 1;
            }
        }
    }

    Ok(rolls)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let width = input.find('\n').ok_or("invalid input")?;
    let mut bytes = input.as_bytes().to_vec();

    let bytes_per_row = width + 1;
    let height = bytes.len() / bytes_per_row;

    let mut removed = 0;
    loop {
        let mut can_be_removed: Vec<usize> = Vec::new();

        for y in 0..height {
            let row_start = y * bytes_per_row;
            for x in 0..width {
                let i = row_start + x;
                if bytes[i] != b'@' {
                    continue;
                }

                let mut adjacent = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
                            continue;
                        }
                        let i = (ny as usize) * bytes_per_row + (nx as usize);
                        if bytes[i] == b'@' {
                            adjacent += 1;
                        }
                    }
                }

                if adjacent < 4 {
                    can_be_removed.push(i);
                }
            }
        }

        if can_be_removed.is_empty() {
            break;
        }

        removed += can_be_removed.len();

        for i in can_be_removed {
            bytes[i] = b'.';
        }
    }

    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(13), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(43), get_part_two(INPUT));
    }
}
