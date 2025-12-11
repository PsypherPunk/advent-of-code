use std::collections::HashMap;

fn find_paths(
    src: &str,
    dest: &str,
    edges: &HashMap<&str, Vec<&str>>,
    memo: &mut HashMap<(String, String), usize>,
) -> usize {
    let key = (src.to_string(), dest.to_string());

    if let Some(&seen) = memo.get(&key) {
        return seen;
    }

    let result = if src == dest {
        1
    } else {
        edges
            .get(src)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .map(|n| find_paths(n, dest, edges, memo))
                    .sum()
            })
            .unwrap_or(0)
    };

    memo.insert(key, result);

    result
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let devices = input
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(": ").ok_or("invalid line")?;
            let outputs = outputs.split_whitespace().collect();

            Ok((device, outputs))
        })
        .collect::<Result<HashMap<_, Vec<_>>, String>>()?;

    Ok(find_paths("you", "out", &devices, &mut HashMap::new()))
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(5), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
