use std::ops::RangeInclusive;

fn get_ranges(input: &str) -> Result<Vec<RangeInclusive<usize>>, String> {
    input
        .trim()
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .ok_or_else(|| "invalid range".to_string())
                .and_then(|(start, end)| {
                    let s = start
                        .parse::<usize>()
                        .map_err(|_| format!("invalid start: {start}"))?;
                    let e = end
                        .parse::<usize>()
                        .map_err(|_| format!("invalid end: {end}"))?;

                    Ok(s..=e)
                })
        })
        .collect::<Result<Vec<RangeInclusive<usize>>, _>>()
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let total = get_ranges(input)?
        .into_iter()
        .map(|range| {
            range
                .map(|id| {
                    let chars = id.to_string().chars().collect::<Vec<_>>();
                    match chars.len() % 2 {
                        0 => {
                            let mut chunks = chars.chunks(chars.len() / 2);
                            match chunks.next() {
                                Some(first) => {
                                    if chunks.all(|chunk| chunk == first) {
                                        id
                                    } else {
                                        0
                                    }
                                }
                                None => 0,
                            }
                        }
                        _ => 0,
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(total)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let total = get_ranges(input)?
        .into_iter()
        .map(|range| {
            range
                .map(|id| {
                    let chars = id.to_string().chars().collect::<Vec<_>>();

                    match (2..=chars.len()).find(|n| match chars.len() % n {
                        0 => {
                            let mut chunks = chars.chunks(chars.len() / n);
                            match chunks.next() {
                                Some(first) => chunks.all(|chunk| chunk == first),
                                None => false,
                            }
                        }
                        _ => false,
                    }) {
                        Some(_) => id,
                        None => 0,
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(1227775554), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(4174379265), get_part_two(INPUT));
    }
}
