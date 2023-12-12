use std::collections::BTreeMap;

fn count_the_ways(
    cache: &mut BTreeMap<(usize, usize, usize), usize>,
    springs: &[u8],
    contiguous_count: Option<usize>,
    remaining_counts: &[usize],
) -> usize {
    if springs.is_empty() {
        return match (contiguous_count, remaining_counts.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining_counts[0] => 1,
            _ => 0,
        };
    }
    if contiguous_count.is_some() && remaining_counts.is_empty() {
        return 0;
    }

    let key = (
        springs.len(),
        contiguous_count.unwrap_or(0),
        remaining_counts.len(),
    );
    if let Some(&ways) = cache.get(&key) {
        return ways;
    }

    let ways = match (springs[0], contiguous_count) {
        (b'.', None) => count_the_ways(cache, &springs[1..], None, remaining_counts),
        (b'.', Some(current_count)) if current_count != remaining_counts[0] => 0,
        (b'.', Some(_)) => count_the_ways(cache, &springs[1..], None, &remaining_counts[1..]),
        (b'#', None) => count_the_ways(cache, &springs[1..], Some(1), remaining_counts),
        (b'#', Some(_)) => count_the_ways(
            cache,
            &springs[1..],
            contiguous_count.map(|count| count + 1),
            remaining_counts,
        ),
        (b'?', None) => {
            count_the_ways(cache, &springs[1..], Some(1), remaining_counts)
                + count_the_ways(cache, &springs[1..], None, remaining_counts)
        }
        (b'?', Some(current_count)) if current_count != remaining_counts[0] => count_the_ways(
            cache,
            &springs[1..],
            Some(current_count + 1),
            remaining_counts,
        ),
        (b'?', Some(current_count)) => {
            count_the_ways(
                cache,
                &springs[1..],
                Some(current_count + 1),
                remaining_counts,
            ) + count_the_ways(cache, &springs[1..], None, &remaining_counts[1..])
        }
        _ => unreachable!(),
    };

    cache.insert(key, ways);

    ways
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut cache = BTreeMap::new();

    let ways = input
        .trim()
        .lines()
        .map(|line| {
            let (springs, sizes) = line
                .split_once(' ')
                .ok_or(format!("bad input: {}", input))?;
            let sizes = sizes
                .split(',')
                .map(|size| size.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;
            let springs = springs.as_bytes();

            cache.clear();

            Ok::<usize, String>(count_the_ways(&mut cache, springs, None, &sizes))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ways.iter().sum())
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(21), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
