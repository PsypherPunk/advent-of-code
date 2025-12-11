use std::collections::HashMap;

fn find_paths(
    start: &str,
    end: &str,
    devices: &HashMap<&str, Vec<&str>>,
    memo: &mut HashMap<(String, String), usize>,
) -> usize {
    let key = (start.to_string(), end.to_string());

    if let Some(&seen) = memo.get(&key) {
        return seen;
    }

    let paths = if start == end {
        1
    } else {
        devices
            .get(start)
            .map(|outputs| {
                outputs
                    .iter()
                    .map(|output| find_paths(output, end, devices, memo))
                    .sum()
            })
            .unwrap_or(0)
    };

    memo.insert(key, paths);

    paths
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

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let devices = input
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(": ").ok_or("invalid line")?;
            let outputs = outputs.split_whitespace().collect();

            Ok((device, outputs))
        })
        .collect::<Result<HashMap<_, Vec<_>>, String>>()?;

    let count = {
        let mut memo = HashMap::new();

        find_paths("svr", "dac", &devices, &mut memo)
            .saturating_mul(find_paths("dac", "fft", &devices, &mut memo))
            .saturating_mul(find_paths("fft", "out", &devices, &mut memo))
            .saturating_add(
                find_paths("svr", "fft", &devices, &mut memo)
                    .saturating_mul(find_paths("fft", "dac", &devices, &mut memo))
                    .saturating_mul(find_paths("dac", "out", &devices, &mut memo)),
            )
    };

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_ONE: &str = r#"aaa: you hhh
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

    const INPUT_PART_TWO: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(5), get_part_one(INPUT_PART_ONE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT_PART_TWO));
    }
}
