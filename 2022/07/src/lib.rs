use std::{collections::HashMap, path::PathBuf};

pub fn get_part_one(input: &str) -> usize {
    let mut cwd = PathBuf::new();

    let sizes = input
        .trim()
        .lines()
        .filter_map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            match parts[0] {
                "$" => match parts[1] {
                    "cd" => match parts[2] {
                        ".." => {
                            cwd.pop();
                            None
                        }
                        _ => {
                            cwd.push(parts[2]);
                            None
                        }
                    },
                    _ => None,
                },
                "dir" => None,
                _ => {
                    cwd.push(parts[1]);
                    let filesize = (cwd.clone(), parts[0].parse::<usize>().unwrap());
                    cwd.pop();

                    Some(filesize)
                }
            }
        })
        .fold(HashMap::new(), |mut acc, (mut path, size)| {
            while path.pop() {
                let entry = acc.entry(path.clone()).or_insert(0);
                *entry += size;
            }

            acc
        });

    sizes
        .values()
        .filter(|&size| *size <= 100_000)
        .sum()
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(95_437, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
