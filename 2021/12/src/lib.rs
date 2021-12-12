use std::collections::{HashMap, HashSet};

fn get_caves(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut paths = HashMap::new();

    input.trim().lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        paths.entry(a).or_insert_with(Vec::new).push(b);
        paths.entry(b).or_insert_with(Vec::new).push(a);
    });

    paths
}

fn get_path_count<'a>(
    current: &'a str,
    caves: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashSet<&'a str>,
) -> usize {
    match current {
        "end" => 1,
        cave => {
            if cave.to_lowercase() == cave && seen.contains(cave) {
                return 0;
            }
            seen.insert(cave);
            caves
                .get(cave)
                .unwrap()
                .iter()
                .map(|&next| get_path_count(next, caves, &mut seen.clone()))
                .sum()
        }
    }
}

fn get_path_count_with_revisit<'a>(
    current: &'a str,
    caves: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashSet<&'a str>,
    mut revisit: Option<&'a str>,
) -> usize {
    match current {
        "end" => 1,
        cave => {
            if cave.to_lowercase() == cave && seen.contains(cave) {
                if revisit.is_none() {
                    revisit = Some(cave);
                } else {
                    return 0;
                }
            }
            if cave == "start" && !seen.is_empty() {
                return 0;
            }
            seen.insert(cave);
            caves
                .get(cave)
                .unwrap()
                .iter()
                .map(|&next| get_path_count_with_revisit(next, caves, &mut seen.clone(), revisit))
                .sum()
        }
    }
}

pub fn get_part_one(input: &str) -> usize {
    let caves = get_caves(input);
    let mut seen = HashSet::new();

    get_path_count("start", &caves, &mut seen)
}

pub fn get_part_two(input: &str) -> usize {
    let caves = get_caves(input);
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
        assert_eq!(paths, get_part_one(input));
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
        assert_eq!(paths, get_part_two(input));
    }
}
