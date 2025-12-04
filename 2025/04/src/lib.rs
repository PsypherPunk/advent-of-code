pub fn get_part_one(input: &str) -> Result<usize, String> {
    let width = input.find('\n').ok_or("invalid input")?;

    let bytes = input.as_bytes();

    let bytes_per_row = width + 1;
    let height = bytes.len() / bytes_per_row;

    let mut occurrences = 0usize;

    for r in 0..height {
        let row_start = r * bytes_per_row;
        for c in 0..width {
            let idx = row_start + c;
            if bytes[idx] != b'@' {
                continue;
            }

            let mut neighbors = 0usize;
            for dr in -1isize..=1 {
                for dc in -1isize..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr < 0 || nr >= height as isize || nc < 0 || nc >= width as isize {
                        continue;
                    }
                    let nidx = (nr as usize) * bytes_per_row + (nc as usize);
                    if bytes[nidx] == b'@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                occurrences += 1;
            }
        }
    }

    Ok(occurrences)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let width = input.find('\n').ok_or("invalid input")?;
    let mut bytes = input.as_bytes().to_vec();

    let bytes_per_row = width + 1;
    let height = bytes.len() / bytes_per_row;

    let mut removed = 0;
    loop {
        let mut to_clear: Vec<usize> = Vec::new();

        for r in 0..height {
            let row_start = r * bytes_per_row;
            for c in 0..width {
                let idx = row_start + c;
                if bytes[idx] != b'@' {
                    continue;
                }

                let mut neighbors = 0usize;
                for dr in -1isize..=1 {
                    for dc in -1isize..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr < 0 || nr >= height as isize || nc < 0 || nc >= width as isize {
                            continue;
                        }
                        let nidx = (nr as usize) * bytes_per_row + (nc as usize);
                        if bytes[nidx] == b'@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_clear.push(idx);
                    removed += 1;
                }
            }
        }

        if to_clear.is_empty() {
            break;
        }

        for idx in to_clear {
            bytes[idx] = b'.';
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
