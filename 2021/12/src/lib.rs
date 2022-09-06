use std::collections::{HashMap, HashSet};

fn get_caves(input: &str) -> Result<HashMap<&str, Vec<&str>>, String> {
    let mut paths = HashMap::new();

    input.trim().lines().try_for_each(|line| {
        let (a, b) = line
            .split_once('-')
            .ok_or_else(|| "invalid input".to_owned())?;

        paths.entry(a).or_insert_with(Vec::new).push(b);
        paths.entry(b).or_insert_with(Vec::new).push(a);

        Ok::<(), String>(())
    })?;

    Ok(paths)
}

fn get_path_count<'a>(
    current: &'a str,
    caves: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashSet<&'a str>,
) -> Result<usize, String> {
    let count = match current {
        "end" => 1,
        cave => {
            if cave.to_lowercase() == cave && seen.contains(cave) {
                return Ok(0);
            }
            seen.insert(cave);
            let count = caves
                .get(cave)
                .ok_or_else(|| "invalid cave".to_owned())?
                .iter()
                .map(|&next| get_path_count(next, caves, &mut seen.clone()))
                .collect::<Result<Vec<_>, String>>()?;
            count.iter().sum()
        }
    };

    Ok(count)
}

fn get_path_count_with_revisit<'a>(
    current: &'a str,
    caves: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashSet<&'a str>,
    mut revisit: Option<&'a str>,
) -> Result<usize, String> {
    let count = match current {
        "end" => 1,
        cave => {
            if cave.to_lowercase() == cave && seen.contains(cave) {
                if revisit.is_none() {
                    revisit = Some(cave);
                } else {
                    return Ok(0);
                }
            }
            if cave == "start" && !seen.is_empty() {
                return Ok(0);
            }
            seen.insert(cave);
            let count = caves
                .get(cave)
                .ok_or_else(|| "invalid cave".to_owned())?
                .iter()
                .map(|&next| get_path_count_with_revisit(next, caves, &mut seen.clone(), revisit))
                .collect::<Result<Vec<_>, String>>()?;

            count.iter().sum()
        }
    };

    Ok(count)
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let caves = get_caves(input)?;
    let mut seen = HashSet::new();

    get_path_count("start", &caves, &mut seen)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let caves = get_caves(input)?;
    let mut seen = HashSet::new();

    get_path_count_with_revisit("start", &caves, &mut seen, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = {
        r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#,
        r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#,
    }, paths = {
        10, 19, 226,
    })]
    fn test_part_one(paths: usize, input: &str) {
        assert_eq!(Ok(paths), get_part_one(input));
    }

    #[parameterized(input = {
        r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#,
        r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#,
    }, paths = {
        36, 103, 3509,
    })]
    fn test_part_two(paths: usize, input: &str) {
        assert_eq!(Ok(paths), get_part_two(input));
    }
}
